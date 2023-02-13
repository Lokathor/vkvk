use super::*;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
extern "system" {
  /// khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Entry {
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  /// Gets the highest API level Instance that can be created on this system.
  ///
  /// Your instance creation request can ask for any API level equal to or less
  /// than this.
  pub fn get_max_instance_version(&self) -> Result<VkVersion, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, vkEnumerateInstanceVersion_NAME.as_ptr()) }) else {
      // When vkEnumerateInstanceVersion is missing we assume that it means Vulkan 1.0
      return Ok(VkVersion::_1_0)
    };
    let vkEnumerateInstanceVersion: vkEnumerateInstanceVersion_t =
      unsafe { core::mem::transmute(pfn) };
    let mut version = VkVersion(0);
    let res = unsafe { vkEnumerateInstanceVersion(&mut version) };
    match res.0 {
      None => Ok(version),
      Some(err_code) => Err(err_code),
    }
  }

  /// Gets the list of available layers and info about them.
  pub fn get_available_layers(&self) -> Result<Vec<VkLayerProperties>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, vkEnumerateInstanceLayerProperties_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkEnumerateInstanceLayerProperties: vkEnumerateInstanceLayerProperties_t =
      unsafe { core::mem::transmute(pfn) };
    //
    let mut property_count: u32 = 0;
    let count_ret = unsafe { vkEnumerateInstanceLayerProperties(&mut property_count, null_mut()) };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(property_count.try_into().unwrap());
    let write_ret =
      unsafe { vkEnumerateInstanceLayerProperties(&mut property_count, buf.as_mut_ptr()) };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(property_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  /// Gets the possible extensions for a given layer.
  ///
  /// When `None` is passed as the layer name you get the extensions available
  /// on the instance with no layers applied.
  pub fn get_available_extensions(
    &self, layer: Option<&ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>>,
  ) -> Result<Vec<VkExtensionProperties>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, vkEnumerateInstanceExtensionProperties_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkEnumerateInstanceExtensionProperties: vkEnumerateInstanceExtensionProperties_t =
      unsafe { core::mem::transmute(pfn) };
    //
    let layer_z: *const u8 = match layer {
      Some(l) => l.0.as_ptr(),
      None => null(),
    };
    //
    let mut extension_count: u32 = 0;
    let count_ret =
      unsafe { vkEnumerateInstanceExtensionProperties(layer_z, &mut extension_count, null_mut()) };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(extension_count.try_into().unwrap());
    let write_ret = unsafe {
      vkEnumerateInstanceExtensionProperties(layer_z, &mut extension_count, buf.as_mut_ptr())
    };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(extension_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  pub fn create_instance(&self, mut request: CreateRequest) -> Result<VkInstance, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, vkCreateInstance_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkCreateInstance: vkCreateInstance_t = unsafe { core::mem::transmute(pfn) };
    //
    request.application_name.push('\0');
    request.engine_name.push('\0');
    let app_create_info = VkApplicationInfo {
      ty: VK_STRUCTURE_TYPE_APPLICATION_INFO,
      next: null(),
      api_version: request.api_version,
      application_name: request.application_name.as_ptr(),
      application_version: request.application_version,
      engine_name: request.engine_name.as_ptr(),
      engine_version: request.engine_version,
    };
    let layer_ptrs: Vec<*const u8> = request.enabled_layers.iter().map(|l| l.as_ptr()).collect();
    let extension_ptrs: Vec<*const u8> =
      request.enabled_extensions.iter().map(|l| l.as_ptr()).collect();
    let instance_create_info = VkInstanceCreateInfo {
      application_info: &app_create_info,
      next: null(),
      ty: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      flags: request.flags,
      enabled_layer_count: layer_ptrs.len().try_into().unwrap(),
      enabled_layer_names: layer_ptrs.as_ptr(),
      enabled_extension_count: extension_ptrs.len().try_into().unwrap(),
      enabled_extension_names: extension_ptrs.as_ptr(),
    };
    //
    let mut instance = VkInstance::NULL;
    let create = unsafe { vkCreateInstance(&instance_create_info, null(), &mut instance) };
    if let Some(err_code) = create.0 {
      Err(err_code)
    } else {
      Ok(instance)
    }
  }
}

pub struct CreateRequest {
  pub application_name: String,
  pub application_version: uint32_t,
  pub engine_name: String,
  pub engine_version: uint32_t,
  pub api_version: VkVersion,
  pub flags: VkInstanceCreateFlags,
  pub enabled_layers: Vec<ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>>,
  pub enabled_extensions: Vec<ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>>,
}
