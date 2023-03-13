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
    let RustFnType { name, params, return_ty, .. } = self;
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
    writeln!(f, "const {name}_NAME:&str = \"{name}\\0\";")?;
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
  writeln!(f, "#[derive(Clone, Copy)]").unwrap();
  writeln!(f, "#[repr(C)]").unwrap();
  writeln!(f, "pub(crate) struct InstanceFns {{").unwrap();
  for instance_entry in instance_entries.iter() {
    let name = instance_entry.name;
    let short_name = name.strip_prefix("vk").unwrap();
    writeln!(f, "  /// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").unwrap();
    if let Some(sync) = instance_entry.implicit_extern_sync_params {
      writeln!(f, "  /// * Implicit Extern Sync: {sync}").unwrap();
    }
    for p in instance_entry.params.iter() {
      let p_name = p.name;
      if let Some(optional) = p.optional {
        writeln!(f, "  /// * `{p_name}` Optional: {optional}").unwrap();
      }
      if let Some(sync) = p.extern_sync {
        writeln!(f, "  /// * `{p_name}` Extern Sync: {sync}").unwrap();
      }
    }
    writeln!(f, "  pub {short_name}: PFN_{name},").unwrap();
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
    if ["VkDevice", "VkCommandPool", "VkQueue"].contains(&p_ty) {
      device_entries.push(fn_ty.clone());
    }
  }
  writeln!(f, "/// pointers for {} fns.", device_entries.len()).unwrap();
  writeln!(f, "#[allow(dead_code)]").unwrap();
  writeln!(f, "#[derive(Clone, Copy)]").unwrap();
  writeln!(f, "#[repr(C)]").unwrap();
  writeln!(f, "pub(crate) struct DeviceFns {{").unwrap();
  for device_entry in device_entries.iter() {
    let name = device_entry.name;
    let short_name = name.strip_prefix("vk").unwrap();
    writeln!(f, "  /// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").unwrap();
    if let Some(sync) = device_entry.implicit_extern_sync_params {
      writeln!(f, "  /// * Implicit Extern Sync: {sync}").unwrap();
    }
    for p in device_entry.params.iter() {
      let p_name = p.name;
      if let Some(optional) = p.optional {
        writeln!(f, "  /// * `{p_name}` Optional: {optional}").unwrap();
      }
      if let Some(sync) = p.extern_sync {
        writeln!(f, "  /// * `{p_name}` Extern Sync: {sync}").unwrap();
      }
    }
    writeln!(f, "  pub {short_name}: PFN_{name},").unwrap();
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
