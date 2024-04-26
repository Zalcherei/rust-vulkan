use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use super::{app_data::AppData, structures::QueueFamilyIndices};

pub unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut AppData) -> Result<vk::CommandPool> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device).unwrap();

    let info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::TRANSIENT)
        .queue_family_index(indices.graphics);

    Ok(device.create_command_pool(&info, None).unwrap())
}
