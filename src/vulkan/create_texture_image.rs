use crate::vulkan::{
    copy_buffer_to_image, create_buffer, create_image, generate_mipmaps, transition_image_layout,
    AppData,
};
use anyhow::Result;
use std::{fs::File, ptr::copy_nonoverlapping as memcpy};
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_texture_image(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Load

    let image = File::open("resources/viking_room.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();
    data.mip_levels = (width.max(height) as f32).log2().floor() as u32 + 1;

    if width != 1024 || height != 1024 || reader.info().color_type != png::ColorType::Rgba {
        panic!("Invalid texture image");
    }

    // Create (staging)

    let (staging_buffer, staging_buffer_memory) = create_buffer(instance, device, data, size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE)?;

    // Copy (staging)

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (image)

    let (texture_image, texture_image_memory) = create_image(instance, device, data, width, height, data.mip_levels, vk::SampleCountFlags::_1, vk::Format::R8G8B8A8_SRGB, vk::ImageTiling::OPTIMAL, vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;

    data.texture_image = texture_image;
    data.texture_image_memory = texture_image_memory;

    // Transition + Copy (image)

    transition_image_layout(device, data, data.texture_image, vk::Format::R8G8B8A8_SRGB, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL, data.mip_levels)?;

    copy_buffer_to_image(device, data, staging_buffer, data.texture_image, width, height)?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    // Mipmaps

    generate_mipmaps(instance, device, data, data.texture_image, vk::Format::R8G8B8A8_SRGB, width, height, data.mip_levels)?;

    Ok(())
}
