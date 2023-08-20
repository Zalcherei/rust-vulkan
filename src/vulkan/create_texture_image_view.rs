use crate::vulkan::{create_image_view, AppData};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_texture_image_view(device: &Device, data: &mut AppData) -> Result<()> {
    data.texture_image_view = create_image_view(device, data.texture_image, vk::Format::R8G8B8A8_SRGB, vk::ImageAspectFlags::COLOR, data.mip_levels)?;

    Ok(())
}
