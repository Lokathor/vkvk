#![allow(unused_imports)]

use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use std::{mem::size_of, num::NonZeroI32};
use vkvk::{
  CreateRequest, Entry, VkInstance, VkInstanceCreateFlagBits, VkPhysicalDeviceFeatures, VkVersion,
};

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win =
    sdl.create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() }).unwrap();

  let mut instance_layers: Vec<String> = Vec::new();
  let instance_extensions: Vec<String> = win
    .get_instance_extensions()
    .unwrap()
    .into_iter()
    .map(|p| {
      let mut s = String::new();
      let mut p = p.cast::<u8>();
      while unsafe { *p } != 0 {
        s.push(unsafe { *p } as char);
        p = unsafe { p.add(1) };
      }
      s
    })
    .collect();

  let Some(f) = win.get_vkGetInstanceProcAddr() else {
    panic!("Couldn't begin Vulkan initialization!");
  };
  let entry = unsafe { Entry::from_fn_ptr(core::mem::transmute(f)) };

  let max_version = entry.get_max_instance_version().unwrap();
  println!("get_max_instance_version: {max_version:?}");

  for layer in entry.get_available_layers().unwrap().into_iter() {
    //println!("Available Layer: {layer:?}");
    let layer_name = layer.layer_name.as_str();
    if cfg!(debug_assertions) && layer_name == "VK_LAYER_KHRONOS_validation" {
      instance_layers.push(String::from(layer_name));
    }
  }

  for _extension in entry.get_available_extensions(None).unwrap().into_iter() {
    //println!("Available Extension: {extension:?}");

    // TODO: set up VK_EXT_debug_utils when available? (and when using
    // validation layers)
  }

  println!("Instance Layers: {instance_layers:?}");
  println!("Instance Extensions: {instance_extensions:?}");

  let instance = entry
    .create_instance(CreateRequest {
      api_version: VkVersion::major_minor_patch(1, 1, 0),
      application_name: String::from("demo"),
      engine_name: String::from("demo engine"),
      flags: VkInstanceCreateFlagBits::default(),
      application_version: 1,
      engine_version: 1,
      instance_extensions,
      instance_layers,
    })
    .unwrap();
  println!("create_instance: {:?}", instance.vk_instance());

  let _surface =
    win.create_surface(unsafe { core::mem::transmute(instance.vk_instance()) }).unwrap();

  let physical_devices = instance.get_physical_devices().unwrap();
  println!("Physical Devices: {physical_devices:?}");
  let physical_device = physical_devices[0];

  //let extension_properties =
  //  instance.get_physical_device_extension_properties(physical_device,
  // None).unwrap(); for extension_property in &extension_properties {
  //  println!("Extension Property: {extension_property:?}");
  //}

  //let physical_device_properties =
  // instance.get_physical_device_properties(physical_device); println!("==
  // {physical_device_properties:?}"); let physical_device_features =
  // instance.get_physical_device_features(physical_device); println!("==
  // {physical_device_features:?}");
  let physical_device_queue_family_properties =
    instance.get_physical_device_queue_family_properties(physical_device);
  //println!(
  //  "== physical_device_queue_family_properties
  // {physical_device_queue_family_properties:?}"
  //);

  let queue_family_index: u32 = physical_device_queue_family_properties
    .iter()
    .position(|prop| prop.queue_flags.graphics())
    .unwrap()
    .try_into()
    .unwrap();
  let features = VkPhysicalDeviceFeatures::default();

  let device = instance
    .create_device(physical_device, &[queue_family_index], Vec::new(), Vec::new(), features)
    .unwrap();
  println!("{:?}", device.vk_device());

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

    // TODO: swap buffers.
  }
}
