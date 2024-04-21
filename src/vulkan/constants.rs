use vulkanalia::{vk, Version};

// The required device extensions
pub const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
// Whether the validation layers should be enabled
pub const VALIDATION_ENABLED: bool = cfg!(debug_assertion);
// The name of the validation layers
pub const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
// the Vulkan SDK version that started requiring the portability subset extension for macOS
pub const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
// The maximum number off frames that can be processed concurrently
pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

// Types
pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;
