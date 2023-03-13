use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};
use vkvk::prelude::*;

fn main() {
  let sdl = Sdl::init(InitFlags::VIDEO);
  let win = sdl
    .create_vk_window(CreateWinArgs { title: "VkVk Example", ..Default::default() })
    .unwrap();
  let entry: Entry = {
    let Some(f) = win.get_vkGetInstanceProcAddr() else {
      panic!("Couldn't obtain vkGetInstanceProcAddr!");
    };
    unsafe { Entry::new(f) }
  };
}
