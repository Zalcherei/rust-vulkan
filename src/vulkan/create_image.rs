use crate::vulkan::{get_memory_type_index, AppData};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_image(instance: &Instance, device: &Device, data: &AppData, width: u32, height: u32, mip_levels: u32, samples: vk::SampleCountFlags, format: vk::Format, tiling: vk::ImageTiling, usage: vk::ImageUsageFlags, properties: vk::MemoryPropertyFlags) -> Result<(vk::Image, vk::DeviceMemory)> {
    // Image

    let info = vk::ImageCreateInfo::builder().image_type(vk::ImageType::_2D).extent(vk::Extent3D { width, height, depth: 1 }).mip_levels(mip_levels).array_layers(1).format(format).tiling(tiling).initial_layout(vk::ImageLayout::UNDEFINED).usage(usage).sharing_mode(vk::SharingMode::EXCLUSIVE).samples(samples);

    let image = device.create_image(&info, None)?;

    // Memory

    let requirements = device.get_image_memory_requirements(image);

    let info = vk::MemoryAllocateInfo::builder().allocation_size(requirements.size).memory_type_index(get_memory_type_index(instance, data, properties, requirements)?);

    let image_memory = device.allocate_memory(&info, None)?;

    device.bind_image_memory(image, image_memory, 0)?;

    Ok((image, image_memory))
}
