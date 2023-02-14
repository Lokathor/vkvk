#![allow(unused_imports)]

use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use std::{mem::size_of, num::NonZeroI32};
use vkvk::{CreateRequest, Entry, VkInstance, VkInstanceCreateFlagBits, VkVersion};

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win =
    sdl.create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() }).unwrap();

  let mut enabled_layers: Vec<String> = Vec::new();
  let enabled_extensions: Vec<String> = win
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
      enabled_layers.push(String::from(layer_name));
    }
  }

  for _extension in entry.get_available_extensions(None).unwrap().into_iter() {
    //println!("Available Extension: {extension:?}");

    // TODO: set up VK_EXT_debug_utils when available? (and when using
    // validation layers)
  }

  let instance = entry
    .create_instance(CreateRequest {
      api_version: VkVersion::major_minor_patch(1, 1, 0),
      application_name: String::from("demo"),
      engine_name: String::from("demo engine"),
      flags: VkInstanceCreateFlagBits::default(),
      application_version: 1,
      engine_version: 1,
      enabled_extensions,
      enabled_layers,
    })
    .unwrap();
  println!("create_instance: {instance:?}");

  let surface = win.create_surface(unsafe { core::mem::transmute(instance) }).unwrap();

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
