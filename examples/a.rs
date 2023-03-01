use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use vkvk::prelude::*;

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win = sdl
    .create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() })
    .unwrap();

  let entry: Entry = {
    let Some(f) = win.get_vkGetInstanceProcAddr() else {
      panic!("Couldn't begin Vulkan initialization!");
    };
    // TODO: make a proper constructor fn?
    unsafe { core::mem::transmute::<_, Entry>(f) }
  };
  let instance: Instance = {
    let mut instance_extensions: Vec<ZString> = win
      .get_instance_extensions()
      .unwrap()
      .into_iter()
      .map(|p| {
        // TODO: wow, haha, this is not good at all!
        let mut s = String::new();
        let mut p = p.cast::<u8>();
        while unsafe { *p } != 0 {
          s.push(unsafe { *p } as char);
          p = unsafe { p.add(1) };
        }
        println!("SDL wants {s}");
        ZString::try_from(s).unwrap()
      })
      .collect();
    let mut instance_layers = Vec::new();

    if cfg!(target_os = "macos") {
      // When on Mac we're running on "MoltenVK", which translates Vulkan to
      // Metal, so we have to enable the portability extension, which (I guess?)
      // lets us get a vulkan that might not be *perfectly* conformant to the
      // "real" vulkan spec.
      let portability = VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME.trim_end();
      if entry
        .enumerate_instance_extension_properties(None)
        .unwrap_or_default()
        .iter()
        .any(|x| x.extension_name.as_str() == portability)
      {
        instance_extensions.push(ZString::try_from(portability).unwrap())
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

    let instance_create_flags = if cfg!(target_os = "macos") {
      VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR
    } else {
      VkInstanceCreateFlagBits::default()
    };

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
        VkVersion::API_1_1,
        instance_create_flags,
        &instance_layer_strs,
        &instance_extension_strs,
      )
      .unwrap()
  };
  let surface: SurfaceKHR = unsafe {
    // this juggles the beryllium vulkan types and the vkvk vulkan types
    let raw = win.create_surface(core::mem::transmute(instance.vk_instance())).unwrap().0;
    let vk_surface_khr = core::mem::transmute::<_, VkSurfaceKHR>(raw);
    instance.raw_surface_handle_to_vkvk_wrapper(vk_surface_khr)
  };
  let physical_device: PhysicalDevice =
    instance.enumerate_physical_devices().unwrap().into_iter().next().unwrap();
  let device: Device = {
    let queue_family_index: u32 = physical_device
      .get_queue_family_properties()
      .iter()
      .position(|prop| (prop.queue_flags & VK_QUEUE_GRAPHICS_BIT).0 != 0)
      .unwrap()
      .try_into()
      .unwrap();

    let mut device_extensions: Vec<ZStr<'_>> =
      vec![ZStr::try_from(VK_KHR_SWAPCHAIN_EXTENSION_NAME).unwrap()];
    if cfg!(target_os = "macos") {
      device_extensions.push(ZStr::try_from("VK_KHR_portability_subset\0").unwrap());
    }
    physical_device.create_device(queue_family_index, &device_extensions, None).unwrap()
  };
  let swapchain: SwapchainKHR = {
    println!("{:?}", physical_device.get_surface_formats_khr(&surface));
    let surface_format = physical_device
      .get_surface_formats_khr(&surface)
      .unwrap()
      .iter()
      .find(|surface_format| {
        surface_format.color_space == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
          && [VK_FORMAT_R8G8B8A8_SRGB, VK_FORMAT_B8G8R8A8_SRGB]
            .contains(&surface_format.format)
      })
      .copied()
      .expect("couldn't find compatible surface format");
    println!("{surface_format:?}");
    let image_usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    let format_properties = physical_device.get_image_format_properties(
      surface_format.format,
      VK_IMAGE_TYPE_2D,
      VK_IMAGE_TILING_OPTIMAL,
      image_usage,
      VkImageCreateFlagBits::default(),
    );
    println!("{format_properties:?}");
    let surface_capabilities =
      physical_device.get_surface_capabilities_khr(&surface).unwrap();
    let surface_present_modes =
      physical_device.get_surface_present_modes_khr(&surface).unwrap();
    let (present_mode, min_image_count) =
      if surface_present_modes.contains(&VK_PRESENT_MODE_MAILBOX_KHR) {
        let min = surface_capabilities.min_image_count;
        let max =
          surface_capabilities.max_image_count.map(NonZeroU32::get).unwrap_or(u32::MAX);
        let count = min.clamp(3, max);
        (VK_PRESENT_MODE_MAILBOX_KHR, count)
      } else if let Some(mode) = surface_present_modes.get(0).copied() {
        (mode, surface_capabilities.min_image_count)
      } else {
        panic!("No presentation modes available!");
      };
    device
      .create_swapchain_khr(
        &surface,
        surface_format,
        surface_capabilities.current_extent,
        present_mode,
        min_image_count,
        image_usage,
      )
      .unwrap()
  };

  // TODO: get queues?

  // TODO: get images and make image_views

  // program "main loop".
  'the_loop: loop {
    // Process pending events.
    #[allow(clippy::never_loop)]
    #[allow(clippy::single_match)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_loop,
        _ => (),
      }
    }

    // TODO: post-events drawing

    // TODO: present an image.
  }

  swapchain.destroy();
  device.destroy();
  surface.destroy();
  instance.destroy();
}
