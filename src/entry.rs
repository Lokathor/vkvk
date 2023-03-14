#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! Provides the [Entry] type.

use crate::prelude::*;

extern "system" {
  fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

/// An entry point to the Vulkan API.
///
/// This allows you to query the limits of [Instance] creation, namely the
/// available layers and extensions. Use this info to create an appropriate
/// instance, which will then allow you to proceed further with initializing
/// Vulkan.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Default for Entry {
  /// Gives [`Entry::LINKED`]
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::LINKED
  }
}
impl Entry {
  /// An entry point that uses the dynamically linked Vulkan.
  ///
  /// This is generally fine to use, but you way wish to dynamically load an
  /// alternate Vulkan entry.
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  /// Wraps the function pointer as a Vulkan entry point.
  ///
  /// ## Safety
  /// * The pointer must be for a genuine [vkGetInstanceProcAddr][vk] function.
  ///
  /// [vk]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
  #[inline]
  #[must_use]
  pub const unsafe fn new(get_instance_proc_addr: vkGetInstanceProcAddr_t) -> Self {
    Self(get_instance_proc_addr)
  }

  /// Gets the maximum instance version that this Vulkan loader supports.
  ///
  /// Khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
  #[inline]
  #[must_use]
  pub fn enumerate_max_instance_version(&self) -> VkVersion {
    const vkEnumerateInstanceVersion_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceVersion\0");
    let opt_f: PFN_vkEnumerateInstanceVersion = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceVersion_NAME.as_ptr(),
      ))
    };
    if let Some(f) = opt_f {
      let mut version = VkVersion::default();
      unsafe { f(&mut version as *mut VkVersion as *mut u32) };
      version
    } else {
      VkVersion::API_1_0
    }
  }

  /// Gets the properties of all available instance layers.
  ///
  /// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
  #[inline]
  pub fn enumerate_instance_layer_properties(
    &self,
  ) -> Result<Vec<VkLayerProperties>, ErrorCode> {
    const vkEnumerateInstanceLayerProperties_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceLayerProperties\0");
    let opt_f: PFN_vkEnumerateInstanceLayerProperties = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceLayerProperties_NAME.as_ptr(),
      ))
    };
    if let Some(f) = opt_f {
      'gather: loop {
        let mut count = 0_u32;
        let r = unsafe { f(&mut count, null_mut()) };
        match r {
          VK_SUCCESS => (),
          other => return Err(ErrorCode::new(other.0).unwrap()),
        }
        let mut buf: Vec<VkLayerProperties> =
          Vec::with_capacity(count.try_into().unwrap());
        let r = unsafe { f(&mut count, buf.as_mut_ptr()) };
        match r {
          VK_SUCCESS => {
            unsafe { buf.set_len(count.try_into().unwrap()) };
            return Ok(buf);
          }
          VK_INCOMPLETE => continue 'gather,
          other => return Err(ErrorCode::new(other.0).unwrap()),
        }
      }
    } else {
      Err(ErrorCode::new(VK_ERROR_UNKNOWN.0).unwrap())
    }
  }

  /// Creates a new [Instance].
  ///
  /// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
  #[inline]
  pub fn create_instance(
    &self, info: &InstanceCreateInfo,
  ) -> Result<Instance, ErrorCode> {
    const vkCreateInstance_NAME: ZStr<'static> = ZStr::from_lit("vkCreateInstance\0");
    let opt_f: PFN_vkCreateInstance = unsafe {
      core::mem::transmute((self.0)(VkInstance::NULL, vkCreateInstance_NAME.as_ptr()))
    };
    let Some(vkCreateInstance) = opt_f else {
      return Err(ErrorCode::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let opt_f: PFN_vkGetInstanceProcAddr = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkGetInstanceProcAddr_NAME.as_ptr(),
      ))
    };
    let Some(vkGetInstanceProcAddr) = opt_f else {
      return Err(ErrorCode::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let mut vk_instance = VkInstance::NULL;
    let r = unsafe {
      vkCreateInstance(
        info as *const InstanceCreateInfo as *const VkInstanceCreateInfo,
        null(),
        &mut vk_instance,
      )
    };
    if let Some(err_code) = ErrorCode::new(r.0) {
      return Err(err_code);
    }
    let mut instance_fns = InstanceFns::default();
    unsafe { instance_fns.load(vk_instance, vkGetInstanceProcAddr) };
    Ok(Instance { vk_instance, fns: Arc::new(instance_fns) })
  }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(missing_docs)]
pub struct ApplicationInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  //
  pub application_name: Option<ZString>,
  pub application_version: u32,
  pub engine_name: Option<ZString>,
  pub engine_version: u32,
  pub api_version: VkVersion,
}
impl Default for ApplicationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_APPLICATION_INFO,
      next: core::ptr::null(),
      application_name: None,
      application_version: Default::default(),
      engine_name: None,
      engine_version: Default::default(),
      api_version: VkVersion::API_1_0,
    }
  }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(missing_docs)]
pub struct InstanceCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkInstanceCreateFlags,
  pub application_info: Option<Box<ApplicationInfo>>,
  enabled_layer_count: u32,
  enabled_layer_names: *mut ZString,
  enabled_extension_count: u32,
  enabled_extension_names: *mut ZString,
  // extra fields
  enabled_layer_capacity: u32,
  enabled_extension_capacity: u32,
}
impl Drop for InstanceCreateInfo {
  fn drop(&mut self) {
    drop(unsafe {
      Vec::from_raw_parts(
        self.enabled_layer_names,
        self.enabled_layer_count.try_into().unwrap(),
        self.enabled_layer_capacity.try_into().unwrap(),
      )
    });
    drop(unsafe {
      Vec::from_raw_parts(
        self.enabled_extension_names,
        self.enabled_extension_count.try_into().unwrap(),
        self.enabled_extension_capacity.try_into().unwrap(),
      )
    });
  }
}
impl Default for InstanceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    let mut layers = ManuallyDrop::new(Vec::new());
    let mut extensions = ManuallyDrop::new(Vec::new());
    Self {
      struct_ty: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      application_info: None,
      enabled_layer_count: layers.len().try_into().unwrap(),
      enabled_layer_capacity: layers.capacity().try_into().unwrap(),
      enabled_layer_names: layers.as_mut_ptr(),
      enabled_extension_count: extensions.len().try_into().unwrap(),
      enabled_extension_capacity: extensions.capacity().try_into().unwrap(),
      enabled_extension_names: extensions.as_mut_ptr(),
    }
  }
}
impl InstanceCreateInfo {
  /// Runs a closure using the layers list.
  ///
  /// ```
  /// # use vkvk::prelude::*;
  /// let zstring = ZString::try_from("hello").unwrap();
  /// let mut instance_create_info = InstanceCreateInfo::default();
  /// instance_create_info.layers_mut(|v| v.push(zstring));
  /// ```
  #[inline]
  pub fn layers_mut<F: FnOnce(&mut Vec<ZString>)>(&mut self, op: F) {
    fake_ptr_len_cap!(
      self.enabled_layer_names,
      self.enabled_layer_count,
      self.enabled_layer_capacity,
      op
    );
  }
  /// Runs a closure using the extensions list.
  ///
  /// ```
  /// # use vkvk::prelude::*;
  /// let zstring = ZString::try_from("hello").unwrap();
  /// let mut instance_create_info = InstanceCreateInfo::default();
  /// instance_create_info.extensions_mut(|v| v.push(zstring));
  /// ```
  #[inline]
  pub fn extensions_mut<F: FnOnce(&mut Vec<ZString>)>(&mut self, op: F) {
    fake_ptr_len_cap!(
      self.enabled_extension_names,
      self.enabled_extension_count,
      self.enabled_extension_capacity,
      op
    );
  }

  /// View the names of the layers to use
  #[inline]
  #[must_use]
  pub fn layers(&self) -> &[ZString] {
    unsafe {
      core::slice::from_raw_parts(
        self.enabled_layer_names,
        self.enabled_layer_count.try_into().unwrap(),
      )
    }
  }
  /// View the names of the extensions to use
  #[inline]
  #[must_use]
  pub fn extensions(&self) -> &[ZString] {
    unsafe {
      core::slice::from_raw_parts(
        self.enabled_extension_names,
        self.enabled_extension_count.try_into().unwrap(),
      )
    }
  }
}
