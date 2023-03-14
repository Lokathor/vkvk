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
  let _instance = {
    let max_instance_version = entry.enumerate_max_instance_version();
    println!("max_instance_version: {max_instance_version:?}",);

    let instance_layer_properties = entry.enumerate_instance_layer_properties().unwrap();
    println!("instance_layer_properties: {instance_layer_properties:?}",);

    let mut instance_layers: Vec<ZString> = Vec::new();
    for instance_layer_property in instance_layer_properties.iter() {
      if cfg!(debug_assertions)
        && instance_layer_property.layer_name.as_zstr() == "VK_LAYER_KHRONOS_validation"
      {
        instance_layers.push(ZString::from(instance_layer_property.layer_name.as_zstr()));
      }
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

    entry.create_instance(&create_info).unwrap();
  };

  'the_main_loop: loop {
    // Process pending events.
    #[allow(clippy::single_match)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_main_loop,
        _ => (),
      }
    }
    // TODO: post-events drawing
    // TODO: present an image.
  }
}
