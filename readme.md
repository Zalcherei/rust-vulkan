# Vulkania vulkan tutorial (rust)

You can read the tutorial [here]("https://kylemayes.github.io/vulkanalia/introduction.html").

Or you can read the original Vulkan tutorial [here]("https://vulkan-tutorial.com/").

```toml
[package]
name = "rust-vulkan"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.68"
lazy_static = "1.4.0"
log = "0.4.17"
nalgebra-glm = "0.18.0"
png = "0.17.7"
pretty_env_logger = "0.4.0"
thiserror = "1.0.38"
tobj = { version = "3.2.3", features = ["log"] }
vulkanalia = { version = "=0.17.0", features = ["libloading", "window"] }
winit = "0.27.5"
```