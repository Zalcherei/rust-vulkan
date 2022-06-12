# Vulkania vulkan tutorial (rust)

You can read the tutorial [here]("https://kylemayes.github.io/vulkanalia/introduction.html").

Or you can read the original Vulkan tutorial [here]("https://vulkan-tutorial.com/").

```toml
[package]
name = "rust-vulkan"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
lazy_static = "1"
log = "0.4"
nalgebra-glm = "0.10"
png = "0.16"
pretty_env_logger = "0.4"
thiserror = "1"
tobj = "2"
vulkanalia = { version = "=0.12.0", features = ["libloading", "window"] }
winit = "0.24"
```
* Note: I haven't tested this Vulkania tutorial with the latest versions of the dependencies.