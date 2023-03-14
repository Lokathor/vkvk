use super::*;

#[derive(Debug, Clone, Default)]
pub struct RustFnParam {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub optional: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub no_auto_validity: bool,
  pub stride: Option<StaticStr>,
  pub object_type: Option<StaticStr>,
  pub valid_structs: Option<StaticStr>,
}

#[derive(Debug, Clone, Default)]
pub struct RustFnType {
  pub name: StaticStr,
  pub params: Vec<RustFnParam>,
  pub return_ty: StaticStr,
  pub comment: Option<StaticStr>,
  pub success_codes: Option<StaticStr>,
  pub error_codes: Option<StaticStr>,
  pub implicit_extern_sync_params: Option<StaticStr>,
  pub queues: Option<StaticStr>,
  pub render_pass: Option<StaticStr>,
  pub cmd_buffer_level: Option<StaticStr>,
  pub tasks: Option<StaticStr>,
  pub video_coding: Option<StaticStr>,
}
impl Display for RustFnType {
  /// * Primary printing puts declarations needed to build a FnTable. This
  ///   includes minimal docs, just a link to the khronos page. The programmer
  ///   won't often see the docs on these type aliases.
  /// * Alternate printing puts the declaration of the field within a FnTable.
  ///   This has full docs, because that's what the programmer will most often
  ///   see via RA.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustFnType {
      name,
      params,
      return_ty,
      comment,
      success_codes,
      error_codes,
      implicit_extern_sync_params,
      queues,
      render_pass,
      cmd_buffer_level,
      tasks,
      video_coding,
    } = self;
    if f.alternate() {
      // this code path prints the fn table field (including field docs).
      let short_name = name.strip_prefix("vk").unwrap();
      writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)")?;
      if let Some(comment) = comment {
        writeln!(f, "///")?;
        writeln!(f, "/// {comment}")?;
      }
      if let Some(success_codes) = success_codes {
        write!(f, "/// * Success: ")?;
        for (i, s) in success_codes.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "`{s}`")?;
        }
        writeln!(f)?;
      }
      if let Some(error_codes) = error_codes {
        write!(f, "/// * Error: ")?;
        for (i, s) in error_codes.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "`{s}`")?;
        }
        writeln!(f)?;
      }
      if let Some(implicit_sync) = implicit_extern_sync_params {
        write!(f, "/// * Implicit Extern Sync: ")?;
        for (i, word) in implicit_sync.split_whitespace().enumerate() {
          if i > 0 {
            write!(f, " ")?;
          }
          if let Some(sname) = word.strip_prefix("sname:") {
            write!(f, "[{sname}]")?;
          } else if let Some(pname) = word.strip_prefix("pname:") {
            write!(f, "`{pname}`")?;
          } else {
            write!(f, "{word}")?;
          }
        }
        writeln!(f)?;
      }
      if let Some(queues) = queues {
        write!(f, "/// * Queues: ")?;
        for (i, s) in queues.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{s}")?;
        }
        writeln!(f)?;
      }
      if let Some(render_pass) = render_pass {
        writeln!(f, "/// * Render Pass: {render_pass}")?;
      }
      if let Some(cmd_buffer_level) = cmd_buffer_level {
        write!(f, "/// * Command Buffer Level: ")?;
        for (i, s) in cmd_buffer_level.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{s}")?;
        }
        writeln!(f)?;
      }
      if let Some(tasks) = tasks {
        write!(f, "/// * Tasks: ")?;
        for (i, s) in tasks.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{s}")?;
        }
        writeln!(f)?;
      }
      if let Some(video_coding) = video_coding {
        writeln!(f, "/// * Video Coding: {video_coding}")?;
      }
      for RustFnParam {
        name,
        ty: _,
        ty_variant: _,
        optional,
        extern_sync,
        len,
        alt_len,
        no_auto_validity,
        stride,
        object_type,
        valid_structs,
      } in params.iter()
      {
        let name = fix_member_name(name);
        write!(f, "/// * `{name}`")?;
        if let Some(optional) = optional {
          write!(f, ", Optional: {optional}")?;
        }
        if let Some(extern_sync) = extern_sync {
          write!(f, ", Extern Sync: {extern_sync}")?;
        }
        if let Some(len) = alt_len.or(*len) {
          let chunks: Vec<String> = len.split_whitespace().map(fix_member_name).collect();
          let len = chunks.join(" ");
          write!(f, ", Len: `{len}`")?;
        }
        if *no_auto_validity {
          write!(f, ", {no_auto_validity}")?;
        }
        if let Some(stride) = stride {
          write!(f, ", Stride: {stride}")?;
        }
        if let Some(object_type) = object_type {
          write!(f, ", Object Type: {object_type}")?;
        }
        if let Some(valid_structs) = valid_structs {
          write!(f, ", Valid Structs: {valid_structs}")?;
        }
        writeln!(f)?;
      }
      writeln!(f, "  pub {short_name}: PFN_{name},")?;
    } else {
      // this code path prints the fn type aliases for the function.
      let param_tys_displays: Vec<String> = params
        .iter()
        .map(|p| {
          format!(
            "{}: {}",
            fix_member_name(p.name),
            format_ty_and_variant(filter_arg_ty(p.ty), p.ty_variant)
          )
        })
        .collect();
      let param_tys: String = param_tys_displays.join(",");
      let return_arrow = match *return_ty {
        "void" => String::new(),
        "uint32_t" => String::from(" -> u32"),
        "uint64_t" => String::from(" -> u64"),
        other => format!(" -> {other}"),
      };
      writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (non-nullable)").unwrap();
      writeln!(
        f,
        "pub type {name}_t = unsafe extern \"system\" fn({param_tys}){return_arrow};"
      )?;
      writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (nullable)").unwrap();
      writeln!(f, "pub type PFN_{name} = Option<{name}_t>;")?;
      writeln!(f, "pub const {name}_NAME:&str = \"{name}\\0\";")?;
    }
    Ok(())
  }
}

pub fn gather_fn_types(registry: &VulkanRegistry) -> BTreeMap<StaticStr, RustFnType> {
  let mut output = BTreeMap::new();
  //
  for Command {
    name,
    params,
    return_ty,
    comment,
    success_codes,
    error_codes,
    implicit_extern_sync_params,
    api,
    queues,
    render_pass,
    cmd_buffer_level,
    tasks,
    video_coding,
  } in registry.commands.iter().cloned()
  {
    if !api_vulkan(api) {
      continue;
    }
    let params: Vec<RustFnParam> = params
      .iter()
      .filter(|p| api_vulkan(p.api))
      .cloned()
      .map(
        |Param {
           name,
           ty,
           ty_variant,
           optional,
           extern_sync,
           len,
           alt_len,
           api: _,
           no_auto_validity,
           stride,
           object_type,
           valid_structs,
         }| RustFnParam {
          name,
          ty,
          ty_variant,
          optional,
          extern_sync,
          len,
          alt_len,
          no_auto_validity,
          stride,
          object_type,
          valid_structs,
        },
      )
      .collect();
    if params.iter().any(|p| BLOCKED_TYPES.contains(&p.ty)) {
      continue;
    }
    let rft = output.entry(name).or_insert(RustFnType::default());
    *rft = RustFnType {
      name,
      params,
      return_ty,
      comment,
      success_codes,
      error_codes,
      implicit_extern_sync_params,
      queues,
      render_pass,
      cmd_buffer_level,
      tasks,
      video_coding,
    };
  }
  //
  output
}

fn filter_arg_ty(ty: &str) -> &str {
  match ty {
    "int" => "c_int",
    "char" => "u8",
    "int32_t" => "i32",
    "uint32_t" => "u32",
    "uint64_t" => "u64",
    "uint16_t" => "u16",
    "float" => "c_float",
    "size_t" => "c_size_t",
    "void" => "c_void",
    "Display" => "XlibDisplay",
    "VisualID" => "XlibVisualID",
    other => other,
  }
}

pub fn write_out_fn_types<P: AsRef<Path>>(
  p: P, fn_tys: &BTreeMap<StaticStr, RustFnType>,
) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f, "#![allow(dead_code)]").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "use crate::prelude::*;").unwrap();
  writeln!(f).unwrap();
  for fn_ty in fn_tys.values() {
    if fn_ty.params.iter().any(|p| BLOCKED_TYPES.contains(&p.ty)) {
      continue;
    }
    writeln!(f, "{fn_ty}").unwrap();
  }
  //
  let mut instance_entries: Vec<RustFnType> = Vec::new();
  for fn_ty in fn_tys.values() {
    let p_ty = fn_ty.params.get(0).map(|p| p.ty).unwrap_or("");
    if ["VkInstance", "VkPhysicalDevice"].contains(&p_ty) {
      instance_entries.push(fn_ty.clone());
    }
  }
  writeln!(f, "/// pointers for {} fns.", instance_entries.len()).unwrap();
  writeln!(f, "#[allow(dead_code)]").unwrap();
  writeln!(f, "#[derive(Clone, Copy, Default)]").unwrap();
  writeln!(f, "#[repr(C)]").unwrap();
  writeln!(f, "pub(crate) struct InstanceFns {{").unwrap();
  for instance_entry in instance_entries.iter() {
    writeln!(f, "  {instance_entry:#}").unwrap();
  }
  writeln!(f, "}}").unwrap();
  writeln!(f, "impl InstanceFns {{").unwrap();
  writeln!(f, "  pub unsafe fn load(&mut self, instance:VkInstance, loader: vkGetInstanceProcAddr_t) {{").unwrap();
  writeln!(f, "    use core::mem::transmute;").unwrap();
  for instance_entry in instance_entries.iter() {
    let name = instance_entry.name;
    let short_name = name.strip_prefix("vk").unwrap();
    writeln!(
      f,
      "    self.{short_name} = transmute(loader(instance, {name}_NAME.as_ptr()));"
    )
    .unwrap();
  }
  writeln!(f, "  }}").unwrap();
  writeln!(f, "}}").unwrap();
  writeln!(f).unwrap();
  //
  let mut device_entries: Vec<RustFnType> = Vec::new();
  for fn_ty in fn_tys.values() {
    let p_ty = fn_ty.params.get(0).map(|p| p.ty).unwrap_or("");
    if ["VkDevice", "VkCommandBuffer", "VkQueue"].contains(&p_ty) {
      device_entries.push(fn_ty.clone());
    }
  }
  writeln!(f, "/// pointers for {} fns.", device_entries.len()).unwrap();
  writeln!(f, "#[allow(dead_code)]").unwrap();
  writeln!(f, "#[derive(Clone, Copy, Default)]").unwrap();
  writeln!(f, "#[repr(C)]").unwrap();
  writeln!(f, "pub(crate) struct DeviceFns {{").unwrap();
  for device_entry in device_entries.iter() {
    writeln!(f, "  {device_entry:#}").unwrap();
  }
  writeln!(f, "}}").unwrap();
  writeln!(f, "impl DeviceFns {{").unwrap();
  writeln!(
    f,
    "  pub unsafe fn load(&mut self, device:VkDevice, loader: vkGetDeviceProcAddr_t) {{"
  )
  .unwrap();
  writeln!(f, "    use core::mem::transmute;").unwrap();
  for device_entry in device_entries.iter() {
    let name = device_entry.name;
    let short_name = name.strip_prefix("vk").unwrap();
    writeln!(
      f,
      "    self.{short_name} = transmute(loader(device, {name}_NAME.as_ptr()));"
    )
    .unwrap();
  }
  writeln!(f, "  }}").unwrap();
  writeln!(f, "}}").unwrap();
  writeln!(f).unwrap();
}
