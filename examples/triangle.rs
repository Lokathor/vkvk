#![allow(clippy::let_unit_value)]

use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use vkvk::prelude::*;
use zstring::ZString;

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win = sdl
    .create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() })
    .unwrap();
  let entry: Entry = {
    let Some(f) = win.get_vkGetInstanceProcAddr() else {
      panic!("Couldn't obtain vkGetInstanceProcAddr!");
    };
    unsafe { Entry::new(f) }
  };
  let instance: Instance = {
    let max_instance_version = entry.enumerate_max_instance_version();
    println!("max_instance_version: {max_instance_version:?}",);

    let instance_layer_properties = entry.enumerate_instance_layer_properties().unwrap();
    println!("instance_layer_properties: {instance_layer_properties:?}",);

    let exts = entry.enumerate_instance_extension_properties(None).unwrap();
    println!("instance_extension_properties(): {exts:?}",);

    let mut instance_layers: Vec<ZString> = Vec::new();
    for instance_layer_property in instance_layer_properties.iter() {
      if cfg!(debug_assertions)
        && instance_layer_property.layer_name.as_zstr() == "VK_LAYER_KHRONOS_validation"
      {
        instance_layers.push(ZString::from(instance_layer_property.layer_name.as_zstr()));
      }
      let layer_name = instance_layer_property.layer_name.as_zstr();
      let exts = entry.enumerate_instance_extension_properties(Some(layer_name)).unwrap();
      println!("instance_extension_properties({layer_name}): {exts:?}",);
    }
    println!("instance_layers: {instance_layers:?}");

    let instance_extensions: Vec<ZString> = win.get_instance_extensions().unwrap();
    println!("instance_extensions: {instance_extensions:?}");

    let mut application_info = Box::<ApplicationInfo>::default();
    application_info.api_version = VkVersion::API_1_1;
    let mut create_info = InstanceCreateInfo::default();
    create_info.application_info = Some(application_info);
    create_info.layers_mut(|v| v.extend(instance_layers.into_iter()));
    create_info.extensions_mut(|v| v.extend(instance_extensions.into_iter()));
    if cfg!(target_os = "macos") {
      create_info.enable_portability();
    }

    entry.create_instance(&create_info).unwrap()
  };
  let _surface: Surface = unsafe {
    // It's unsafe to create the surface (we have to pass a valid instance handle)
    let vk_surface_khr = win.create_surface(instance.vk_instance()).unwrap();
    // It's unsafe to mark a raw surface as a child of our instance (it has to have
    // *really* been made from our instance).
    instance.make_raw_surface_a_child_of_this(vk_surface_khr)
  };
  let physical_device: PhysicalDevice = {
    let mut physical_devices = instance.enumerate_physical_devices().unwrap();
    println!("physical_devices: {physical_devices:?}");
    physical_devices.retain(|pd| {
      let ext_props = pd.enumerate_device_extension_properties().unwrap_or_default();
      let supports_swapchain =
        ext_props.iter().any(|p| p.extension_name == VK_KHR_SWAPCHAIN_EXTENSION_NAME);
      supports_swapchain
    });
    physical_devices.into_iter().next().expect("No valid physical devices!")
  };
  let _device: Device = {
    let device_extensions = vec![
      ZString::from(VK_KHR_SWAPCHAIN_EXTENSION_NAME),
      #[cfg(target_os = "macos")]
      ZString::from(VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME),
    ];
    let extensions = zstrings_as_zstrs(&device_extensions);
    let features = None;
    let needs_graphics = true;
    physical_device.create_device(extensions, features, needs_graphics).unwrap()
  };

  // TODO: get the queues

  // TODO: make the swapchain

  // TODO: get the images

  // TODO: get the image views

  // TODO: shader modules

  // TODO: render passes

  // TODO: graphics pipeline

  // TODO: framebuffers

  // TODO: command pool

  // TODO: command buffer

  // TODO: presentation

  'the_main_loop: loop {
    // Process pending events.
    #[allow(clippy::single_match)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_main_loop,
        _ => (),
      }
    }
  }
}

// TODO: === post-triangle goals ===

// TODO: window resizing

// TODO: vertex buffers

// TODO: uniform buffers

// TODO: texture mapping

// TODO: depth buffer

// TODO: model loading

// TODO: mipmaps

// TODO: multisampling

// TODO: compute shaders
