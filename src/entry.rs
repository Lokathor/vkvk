#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! Provides the [Entry] type, and related helper types.

use crate::prelude::*;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
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
  ) -> Result<Vec<VkLayerProperties>, VkError> {
    const vkEnumerateInstanceLayerProperties_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceLayerProperties\0");
    let opt_f: PFN_vkEnumerateInstanceLayerProperties = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceLayerProperties_NAME.as_ptr(),
      ))
    };
    let Some(vkEnumerateInstanceLayerProperties) = opt_f else {
      let err_code = VkError::new(VK_ERROR_UNKNOWN.0).unwrap();
      return Err(err_code);
    };
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe { vkEnumerateInstanceLayerProperties(&mut count, null_mut()) };
      match r {
        VK_SUCCESS => (),
        other => return Err(VkError::new(other.0).unwrap()),
      }
      let mut buf: Vec<VkLayerProperties> = Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe { vkEnumerateInstanceLayerProperties(&mut count, buf.as_mut_ptr()) };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(buf);
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }

  /// Get the properties of all extensions provided by the given layer.
  ///
  /// If you supply `None` then you get info on extensions that are available
  /// without any extra layers being enabled.
  #[inline]
  pub fn enumerate_instance_extension_properties(
    &self, layer: Option<ZStr<'_>>,
  ) -> Result<Vec<VkExtensionProperties>, VkError> {
    const vkEnumerateInstanceExtensionProperties_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceExtensionProperties\0");
    let opt_f: PFN_vkEnumerateInstanceExtensionProperties = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceExtensionProperties_NAME.as_ptr(),
      ))
    };
    let Some(vkEnumerateInstanceExtensionProperties) = opt_f else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let layer_name: *const u8 = layer.map(ZStr::as_ptr).unwrap_or(null());
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkEnumerateInstanceExtensionProperties(layer_name, &mut count, null_mut())
      };
      match r {
        VK_SUCCESS => (),
        other => return Err(VkError::new(other.0).unwrap()),
      }
      let mut buf: Vec<VkExtensionProperties> =
        Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkEnumerateInstanceExtensionProperties(layer_name, &mut count, buf.as_mut_ptr())
      };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(buf);
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }

  /// Creates a new [Instance].
  ///
  /// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
  ///
  /// ## Panics
  /// * If the bit for `VK_KHR_portability_enumeration` is set within the flags,
  ///   but not listed as an enabled extension, this will panic.
  #[inline]
  pub fn create_instance(&self, info: &InstanceCreateInfo) -> Result<Instance, VkError> {
    if info.flags.enumerate_portability() {
      let extension_enabled = info
        .extensions()
        .iter()
        .any(|z| *z == VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME);
      assert!(extension_enabled, "VUID-VkInstanceCreateInfo-flags-06559: portability bit set but extension not listed");
    }
    //
    const vkCreateInstance_NAME: ZStr<'static> = ZStr::from_lit("vkCreateInstance\0");
    let opt_f: PFN_vkCreateInstance = unsafe {
      core::mem::transmute((self.0)(VkInstance::NULL, vkCreateInstance_NAME.as_ptr()))
    };
    let Some(vkCreateInstance) = opt_f else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let opt_f: PFN_vkGetInstanceProcAddr = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkGetInstanceProcAddr_NAME.as_ptr(),
      ))
    };
    let Some(vkGetInstanceProcAddr) = opt_f else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let mut vk_instance = VkInstance::NULL;
    let r = unsafe {
      vkCreateInstance(
        info as *const InstanceCreateInfo as *const VkInstanceCreateInfo,
        null(),
        &mut vk_instance,
      )
    };
    if let Some(err_code) = VkError::new(r.0) {
      return Err(err_code);
    }
    let mut instance_fns = InstanceFns::default();
    unsafe { instance_fns.load(vk_instance, vkGetInstanceProcAddr) };
    Ok(Instance(Arc::new(DestroyInstanceOnDrop {
      vk_instance,
      fns: Arc::new(instance_fns),
    })))
  }
}

/// Allows you to give info about your application during Instance creation.
///
/// Khronos: [VkApplicationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct ApplicationInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  /// Your application's name. The driver probably doesn't care unless you're a
  /// AAA game studio.
  pub application_name: Option<ZString>,
  /// The version of your application. Again, the driver probably doesn't really
  /// care.
  pub application_version: u32,
  /// The name of your game engine, which doesn't matter.
  pub engine_name: Option<ZString>,
  /// The engine's version, which doesn't matter.
  pub engine_version: u32,
  /// This is the maximum level of API that your application is intended for.
  /// You **should** set this, otherwise the maximum allowed API level of your
  /// application will by stick at only 1.0.
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

/// Info for an Instance creation request.
///
/// Khronos: [VkInstanceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct InstanceCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkInstanceCreateFlags,
  /// The application info. You **should** provide this and fill in at least the
  /// application info's `api_version` field.
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
  #[inline]
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
  /// Enable [VK_KHR_portability_enumeration](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_enumeration.html)
  ///
  /// This sets the appropriate bit within the flags field, and adds the
  /// extension name to the extensions list.
  #[inline]
  pub fn enable_portability(&mut self) {
    self.flags |= VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
    self.layers_mut(|v| {
      let already_listed =
        v.iter().any(|z| *z == VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME);
      if !already_listed {
        v.push(ZString::from(VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME))
      }
    });
  }

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
