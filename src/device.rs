#![allow(non_snake_case)]

use super::*;

pub struct Device {
  pub(crate) entry: Entry,
  pub(crate) vk_instance: VkInstance,
  pub(crate) vk_device: VkDevice,
}

impl Device {
  #[inline]
  #[must_use]
  pub const fn vk_device(&self) -> VkDevice {
    self.vk_device
  }
}
