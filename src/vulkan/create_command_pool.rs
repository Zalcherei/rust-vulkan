use crate::vulkan::{AppData, QueueFamilyIndices};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut AppData) -> Result<vk::CommandPool> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder().flags(vk::CommandPoolCreateFlags::TRANSIENT).queue_family_index(indices.graphics);

    Ok(device.create_command_pool(&info, None)?)
}
