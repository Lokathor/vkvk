use std::env::var;

fn main() {
  let target_family = var("CARGO_CFG_TARGET_FAMILY").unwrap();
  let target_pointer_width = var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
  if let Ok(sdk) = var("VULKAN_SDK") {
    let sub_folder = match (target_family.as_str(), target_pointer_width.as_str()) {
      ("windows", "32") => "Lib32",
      ("windows", "64") => "Lib",
      _ => "lib",
    };
    let search_path = std::path::Path::new(&sdk).join(std::path::Path::new(&sub_folder));
    println!("cargo:rustc-link-search={}", search_path.display());
  }
  println!("cargo:rerun-if-env-changed=VULKAN_SDK");
}
