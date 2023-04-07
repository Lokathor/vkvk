#![allow(clippy::let_unit_value)]

use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use bytemuck::pod_collect_to_vec;
use vkvk::prelude::*;
use zstring::ZString;

fn main() {
  let vert_bytes: Vec<u32> =
    pod_collect_to_vec(&std::fs::read("target/shader1.vert.spv").unwrap());
  let frag_bytes: Vec<u32> =
    pod_collect_to_vec(&std::fs::read("target/shader1.frag.spv").unwrap());
  //
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
  let surface: Surface = unsafe {
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
  let device: Device = {
    let device_extensions = vec![
      ZString::from(VK_KHR_SWAPCHAIN_EXTENSION_NAME),
      #[cfg(target_os = "macos")]
      ZString::from(VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME),
    ];
    let extensions = zstrings_as_zstrs(&device_extensions);
    let features = None;
    let target_surface = Some(&surface);
    physical_device.create_device(extensions, features, target_surface).unwrap()
  };
  let swapchain: Swapchain = device.create_swapchain_khr(&surface).unwrap();
  println!(
    "Swapchain: min images {}, {:?}, {:?}, {:?}",
    swapchain.min_image_count(),
    swapchain.image_extent(),
    swapchain.surface_format(),
    swapchain.present_mode()
  );
  let (graphics_pipeline, graphics_pipeline_layout, render_pass) = {
    let vert_shader_module = device.create_shader_module(&vert_bytes).unwrap();
    let frag_shader_module = device.create_shader_module(&frag_bytes).unwrap();
    let shader_stage_create_info = [
      PipelineShaderStageCreateInfo::new(
        ShaderStage::Vertex,
        &vert_shader_module,
        ZString::try_from("main").unwrap(),
      ),
      PipelineShaderStageCreateInfo::new(
        ShaderStage::Fragment,
        &frag_shader_module,
        ZString::try_from("main").unwrap(),
      ),
    ];
    let mut create_info =
      GraphicsPipelineCreateInfo::from_shader_stage_info_slice(&shader_stage_create_info);
    create_info.dynamic_state =
      Some(Box::new(PipelineDynamicStateCreateInfo::from_hash_set(
        vec![VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR].into_iter().collect(),
      )));
    create_info.vertex_input_state = Some(Box::default());
    create_info.input_assembly_state =
      Some(Box::new(PipelineInputAssemblyStateCreateInfo::new(
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        false,
      )));
    create_info.viewport_state =
      Some(Box::new(PipelineViewportStateCreateInfo::new_dynamic(1, 1)));
    create_info.rasterization_state = Some({
      let mut b = Box::<PipelineRasterizationStateCreateInfo>::default();
      b.polygon_mode = VK_POLYGON_MODE_FILL;
      b.line_width = 1.0;
      b.cull_mode = VK_CULL_MODE_BACK_BIT;
      b.front_face = VK_FRONT_FACE_CLOCKWISE;
      b
    });
    create_info.multisample_state = Some(Box::default());
    create_info.color_blend_state = Some(Box::default());
    create_info.layout = unsafe {
      let flags = VkPipelineLayoutCreateFlags::default();
      let descriptor_set_layouts = &[];
      let push_constant_ranges = &[];
      device
        .create_pipeline_layout(flags, descriptor_set_layouts, push_constant_ranges)
        .unwrap()
    };
    create_info.render_pass = unsafe {
      let attachments = &[VkAttachmentDescription {
        format: swapchain.surface_format().format,
        samples: VK_SAMPLE_COUNT_1_BIT,
        load_op: VK_ATTACHMENT_LOAD_OP_CLEAR,
        store_op: VK_ATTACHMENT_STORE_OP_STORE,
        stencil_load_op: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        stencil_store_op: VK_ATTACHMENT_STORE_OP_DONT_CARE,
        initial_layout: VK_IMAGE_LAYOUT_UNDEFINED,
        final_layout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        ..Default::default()
      }];
      let attachment_ref = VkAttachmentReference {
        attachment: 0,
        layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
      };
      let subpasses = &[VkSubpassDescription {
        pipeline_bind_point: VK_PIPELINE_BIND_POINT_GRAPHICS,
        color_attachment_count: 1,
        color_attachments: &attachment_ref,
        ..Default::default()
      }];
      let dependencies = &[];
      device.vk_create_render_pass(attachments, subpasses, dependencies).unwrap()
    };
    create_info.subpass = 0; // ??
    (
      unsafe { device.create_graphics_pipeline(&create_info) }.unwrap(),
      create_info.layout,
      create_info.render_pass,
    )
  };

  let mut framebuffers = Vec::new();
  for image_view in swapchain.image_views() {
    let extent = swapchain.image_extent();
    framebuffers
      .push(device.create_framebuffer(&render_pass, image_view, extent).unwrap());
  }

  // TODO: command pool

  // TODO: command buffer

  'the_main_loop: loop {
    // Process pending events.
    #[allow(clippy::single_match)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_main_loop,
        _ => (),
      }
    }

    // TODO: draw + presentation
  }

  unsafe {
    for framebuffer in framebuffers.drain(..) {
      device.destroy_framebuffer(framebuffer);
    }
    device.vk_destroy_render_pass(render_pass);
    device.vk_destroy_pipeline_layout(graphics_pipeline_layout);
    device.vk_destroy_pipeline(graphics_pipeline);
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
