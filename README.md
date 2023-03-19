# [Docs.rs](https://docs.rs/vkvk)

# vkvk

A vulkan library.

## Running The Example

The shaders aren't compiled as part of the build script.
Instead, run `shader_build.bat` (or copy the commands out and run them in your unix terminal).
This will require that the `glslc` program be installed, which is part of the normal Vulkan SDK.
The compiled SPIRV data will be put into the `target/` directory.
Remember: If you run `cargo clean` the `target/` directory will get deleted and you'll need to compile the shaders again.

After that, `cargo run --example triangle` should work.
