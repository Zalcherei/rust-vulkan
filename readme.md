# Vulkania vulkan tutorial (rust)

[You can read the tutorial](https://kylemayes.github.io/vulkanalia/introduction.html).

Or you can read the original [Vulkan tutorial](https://vulkan-tutorial.com/).

```toml
[package]
name = "rust-vulkan"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow              = "1"
cgmath              = "0.18"
log                 = "0.4"
png                 = "0.17"
pretty_env_logger   = "0.5"
thiserror           = "1"
tobj                = "4"
vulkanalia          = { version = "=0.23.0", features = ["libloading", "provisional", "window"] }
winit               = "0.29"
```
