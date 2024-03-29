use crate::vulkan::{AppData, QueueFamilyIndices};
use anyhow::Result;
use std::collections::HashSet;
use vulkanalia::{
    prelude::v1_0::*,
    Version
};

// Whether the validation layers should be enabled
const VALIDATION_ENABLED: bool = cfg!(debug_assertion);
// The name of the validation layers
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

// The required device extensions
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
// the Vulkan SDK version that started requiring the portability subset extension for macOS
pub const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

pub unsafe fn create_logical_device(entry: &Entry, instance: &Instance, data: &mut AppData) -> Result<Device> {
    // Queue Create Infos

    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices.iter().map(|i| {
        vk::DeviceQueueCreateInfo::builder().queue_family_index(*i).queue_priorities(queue_priorities)
    }).collect::<Vec<_>>();

    // Layers

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    // Extensions

    let mut extensions = DEVICE_EXTENSIONS.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
    }

    // Features

    let features = vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true).sample_rate_shading(true);

    // Create

    let info = vk::DeviceCreateInfo::builder().queue_create_infos(&queue_infos).enabled_layer_names(&layers).enabled_extension_names(&extensions).enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    // Queues

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}