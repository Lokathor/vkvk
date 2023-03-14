//! Provides the [Instance] type.

#[allow(unused_imports)]
use crate::prelude::*;

/// ?
pub struct Instance {
  pub(crate) vk_instance: VkInstance,
  pub(crate) fns: Arc<InstanceFns>,
}
