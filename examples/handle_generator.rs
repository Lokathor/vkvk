/// cargo run --example handle_generator >target/handles.rs
fn main() {
  handle("VkInstance", None);
  handle("VkPhysicalDevice", Some("VkInstance"));
  handle("VkDevice", Some("VkPhysicalDevice"));
  handle("VkQueue", Some("VkDevice"));
  handle("VkCommandBuffer", Some("VkCommandPool"));
  //
  nondispatchable_handle("VkDeviceMemory", Some("VkDevice"));
  nondispatchable_handle("VkCommandPool", Some("VkDevice"));
  nondispatchable_handle("VkBuffer", Some("VkDevice"));
  nondispatchable_handle("VkBufferView", Some("VkDevice"));
  nondispatchable_handle("VkImage", Some("VkDevice"));
  nondispatchable_handle("VkImageView", Some("VkDevice"));
  nondispatchable_handle("VkShaderModule", Some("VkDevice"));
  nondispatchable_handle("VkPipeline", Some("VkDevice"));
  nondispatchable_handle("VkPipelineLayout", Some("VkDevice"));
  nondispatchable_handle("VkSampler", Some("VkDevice"));
  nondispatchable_handle("VkDescriptorSet", Some("VkDescriptorPool"));
  nondispatchable_handle("VkDescriptorSetLayout", Some("VkDevice"));
  nondispatchable_handle("VkDescriptorPool", Some("VkDevice"));
  nondispatchable_handle("VkFence", Some("VkDevice"));
  nondispatchable_handle("VkSemaphore", Some("VkDevice"));
  nondispatchable_handle("VkEvent", Some("VkDevice"));
  nondispatchable_handle("VkQueryPool", Some("VkDevice"));
  nondispatchable_handle("VkFramebuffer", Some("VkDevice"));
  nondispatchable_handle("VkRenderPass", Some("VkDevice"));
  nondispatchable_handle("VkPipelineCache", Some("VkDevice"));
  nondispatchable_handle("VkIndirectCommandsLayoutNV", Some("VkDevice"));
  nondispatchable_handle("VkDescriptorUpdateTemplate", Some("VkDevice"));
  nondispatchable_handle("VkValidationCacheEXT", Some("VkDevice"));
  nondispatchable_handle("VkAccelerationStructureKHR", Some("VkDevice"));
  nondispatchable_handle("VkAccelerationStructureNV", Some("VkDevice"));
  nondispatchable_handle("VkPerformanceConfigurationINTEL", Some("VkDevice"));
  nondispatchable_handle("VkBufferCollectionFUCHSIA", Some("VkDevice"));
  nondispatchable_handle("VkDeferredOperationKHR", Some("VkDevice"));
  nondispatchable_handle("VkPrivateDataSlot", Some("VkDevice"));
  nondispatchable_handle("VkCuModuleNVX", Some("VkDevice"));
  nondispatchable_handle("VkCuFunctionNVX", Some("VkDevice"));
  nondispatchable_handle("VkOpticalFlowSessionNV", Some("VkDevice"));
  nondispatchable_handle("VkMicromapEXT", Some("VkDevice"));

  // /* WSI extensions
  nondispatchable_handle("VkDisplayKHR", Some("VkPhysicalDevice"));
  nondispatchable_handle("VkDisplayModeKHR", Some("VkDisplayKHR"));
  nondispatchable_handle("VkSurfaceKHR", Some("VkInstance"));
  nondispatchable_handle("VkSwapchainKHR", Some("VkDevice"));
  nondispatchable_handle("VkDebugReportCallbackEXT", Some("VkInstance"));
  nondispatchable_handle("VkDebugUtilsMessengerEXT", Some("VkInstance"));
  // */

  /* Video extensions
  nondispatchable_handle("VkVideoSessionKHR");
  nondispatchable_handle("VkVideoSessionParametersKHR");
  */
}

fn handle(name: &str, parent: Option<&str>) {
  println!("/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (handle)");
  if let Some(parent) = parent {
    println!("/// * Parent: [{parent}]");
  }
  println!(
    "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct {name}(*mut core::ffi::c_void);
    unsafe impl Send for {name} {{ }}
    unsafe impl Sync for {name} {{ }}
    impl Default for {name} {{
      #[inline]
      #[must_use]
      fn default() -> Self {{
        Self::NULL
      }}
    }}
    impl {name} {{
      pub const NULL: Self = Self::null();
      #[inline]
      #[must_use]
      pub const fn null() -> Self {{
        Self(core::ptr::null_mut())
      }}
    }}
    "
  )
}

fn nondispatchable_handle(name: &str, parent: Option<&str>) {
  println!("/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (non-dispatchable handle)");
  if let Some(parent) = parent {
    println!("/// * Parent: [{parent}]");
  }
  println!(
    "#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct {name}(u64);
    impl {name} {{
      pub const ZERO: Self = Self::zero();
      #[inline]
      #[must_use]
      pub const fn zero() -> Self {{
        Self(0)
      }}
    }}
    "
  )
}
