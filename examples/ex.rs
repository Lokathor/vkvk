#![allow(unused_imports)]

use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use std::{mem::size_of, num::NonZeroI32};
use vkvk::{
  CreateRequest, Entry, VkImageCreateFlagBits, VkImageCreateFlags, VkImageUsageFlags, VkInstance,
  VkInstanceCreateFlagBits, VkPhysicalDeviceFeatures, VkSurfaceKHR, VkVersion,
  VK_COLOR_SPACE_SRGB_NONLINEAR_KHR, VK_FORMAT_B8G8R8A8_SRGB, VK_FORMAT_R8G8B8A8_SRGB,
  VK_IMAGE_TILING_OPTIMAL, VK_IMAGE_TYPE_2D, VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
  VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR, VK_PRESENT_MODE_MAILBOX_KHR,
};

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win =
    sdl.create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() }).unwrap();

  let mut instance_layers: Vec<String> = Vec::new();
  let mut instance_extensions: Vec<String> = win
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

  for extension in entry.get_available_extensions(None).unwrap().into_iter() {
    //println!("Available Extension: {extension:?}");
    let extension_name = extension.extension_name.as_str();
    if cfg!(target_os = "macos") && extension_name == "VK_KHR_portability_enumeration" {
      instance_extensions.push(String::from(extension_name))
    }
  }

  println!("Requesting Instance Layers: {instance_layers:?}");
  println!("Requesting Instance Extensions: {instance_extensions:?}");

  let instance_create_flags = if cfg!(target_os = "macos") {
    VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR
  } else {
    VkInstanceCreateFlagBits::default()
  };

  let instance = entry
    .create_instance(CreateRequest {
      api_version: VkVersion::major_minor_patch(1, 1, 0),
      application_name: String::from("demo"),
      engine_name: String::from("demo engine"),
      instance_create_flags,
      application_version: 1,
      engine_version: 1,
      instance_extensions,
      instance_layers,
    })
    .unwrap();
  //println!("create_instance: {:?}", instance.vk_instance());

  let surface: VkSurfaceKHR = unsafe {
    // this juggles the beryllium vulkan types and the vkvk vulkan types
    let u: u64 = win.create_surface(core::mem::transmute(instance.vk_instance())).unwrap().0;
    core::mem::transmute(u)
  };

  let mut physical_devices = instance.get_physical_devices().unwrap();
  print!(
    "There {are_is} {} physical device{s} on the system: ",
    physical_devices.len(),
    are_is = if physical_devices.len() != 1 { "are" } else { "is" },
    s = if physical_devices.len() != 1 { "s" } else { "" },
  );
  for physical_device in physical_devices.iter() {
    let properties = instance.get_physical_device_properties(*physical_device);
    let name = properties.device_name.as_str();
    let version = properties.api_version;
    print!("{name} ({version}),")
  }
  println!();
  physical_devices.retain(|&physical_device| {
    // the device needs to support making a swapchain
    //let x = instance.get_physical_device_properties(physical_device);
    //println!("=== {x:?}");
    let extension_properties =
      instance.get_physical_device_extension_properties(physical_device, None).unwrap();
    extension_properties.iter().any(|ex| ex.extension_name.as_str() == "VK_KHR_swapchain")
  });
  let physical_device =
    physical_devices.into_iter().next().expect("No physical devices supported our requirements!");

  let physical_device_queue_family_properties =
    instance.get_physical_device_queue_family_properties(physical_device);
  let queue_family_index: u32 = physical_device_queue_family_properties
    .iter()
    .position(|prop| prop.queue_flags.graphics())
    .unwrap()
    .try_into()
    .unwrap();
  let features = VkPhysicalDeviceFeatures::default();
  let device_layers = Vec::new();
  let mut device_extensions = vec![String::from("VK_KHR_swapchain")];
  if cfg!(target_os = "macos") {
    device_extensions.push(String::from("VK_KHR_portability_subset"));
  }

  let physical_device_surface_capabilities =
    instance.get_physical_device_surface_capabilities_khr(physical_device, surface).unwrap();
  //println!("{physical_device_surface_capabilities:?}");

  let physical_device_surface_formats =
    instance.get_physical_device_surface_formats(physical_device, surface).unwrap();
  //println!("{physical_device_surface_formats:?}");
  let surface_format = physical_device_surface_formats
    .iter()
    .find(|surface_format| {
      surface_format.color_space == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
        && [VK_FORMAT_R8G8B8A8_SRGB, VK_FORMAT_B8G8R8A8_SRGB].contains(&surface_format.format)
    })
    .copied()
    .expect("No compatible surface formats found!");

  let physical_device_surface_present_modes =
    instance.get_physical_device_surface_present_modes(physical_device, surface).unwrap();
  //println!("{physical_device_surface_present_modes:?}");
  let (present_mode, min_image_count) =
    if physical_device_surface_present_modes.contains(&VK_PRESENT_MODE_MAILBOX_KHR) {
      (
        VK_PRESENT_MODE_MAILBOX_KHR,
        physical_device_surface_capabilities
          .min_image_count
          .clamp(3, physical_device_surface_capabilities.max_image_count),
      )
    } else if let Some(mode) = physical_device_surface_present_modes.get(0).copied() {
      (mode, physical_device_surface_capabilities.min_image_count)
    } else {
      panic!("No presentation modes available!");
    };
  //println!("present_mode: {present_mode:?}, min_image_count:
  // {min_image_count:?}");

  let image_usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

  let _physical_device_image_format_properties = instance
    .get_physical_device_image_format_properties(
      physical_device,
      surface_format.format,
      VK_IMAGE_TYPE_2D,
      VK_IMAGE_TILING_OPTIMAL,
      image_usage,
      VkImageCreateFlags::default(),
    );
  //println!("physical_device_image_format_properties:
  // {physical_device_image_format_properties:?}");

  let device = instance
    .create_device(
      physical_device,
      &[queue_family_index],
      device_layers,
      device_extensions,
      features,
    )
    .unwrap();
  //println!("Created our device!");
  //println!("{:?}", device.vk_device());
  //println!("{:?}", device.queues());

  let swapchain = device
    .create_swapchain(
      surface,
      surface_format,
      physical_device_surface_capabilities.current_extent,
      present_mode,
      min_image_count,
      image_usage,
    )
    .unwrap();
  //println!("Created our swapchain! {swapchain:?}");

  // TODO: swapchain images

  // TODO: swapchain views

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

  device.destroy_swapchain(swapchain);
  device.destroy_device();
  instance.destroy_surface(surface);
  instance.destroy_instance();
}
