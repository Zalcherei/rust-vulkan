# Vulkania vulkan tutorial (rust)

[You can read the tutorial]("https://kylemayes.github.io/vulkanalia/introduction.html").

Or you can read the original [Vulkan tutorial]("https://vulkan-tutorial.com/").

```toml
[package]
name = "rust-vulkan"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.72"
lazy_static = "1.4.0"
log = "0.4.19"
nalgebra-glm = "0.18.0"
png = "0.17.9"
pretty_env_logger = "0.5.0"
thiserror = "1.0.44"
tobj = { version = "4.0.0", features = ["log"] }
vulkanalia = { version = "=0.21.0", features = ["libloading", "window"] }
winit = "0.28.6"
```