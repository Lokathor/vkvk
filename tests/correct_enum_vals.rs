use vkvk::prelude::*;

#[test]
fn test_vk_khr_surface() {
  assert_eq!(VK_ERROR_SURFACE_LOST_KHR.0, -1000000000_i32);
  assert_eq!(VK_ERROR_NATIVE_WINDOW_IN_USE_KHR.0, -1000000001_i32);
}

#[test]
fn test_vk_khr_swapchain() {
  assert_eq!(VK_SUBOPTIMAL_KHR.0, 1000001003_i32);
  assert_eq!(VK_ERROR_OUT_OF_DATE_KHR.0, -1000001004_i32);
}
