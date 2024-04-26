use super::{app_data::AppData, get_memory_type_index::get_memory_type_index};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_buffer(
    instance: &Instance,
    device: &Device,
    data: &AppData,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    // Buffer

    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = device.create_buffer(&buffer_info, None).unwrap();

    // Memory

    let requirements = device.get_buffer_memory_requirements(buffer);

    let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(instance, data, properties, requirements).unwrap());

    let buffer_memory = device.allocate_memory(&memory_info, None).unwrap();

    device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

    Ok((buffer, buffer_memory))
}
