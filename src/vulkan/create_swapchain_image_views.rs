use super::{app_data::AppData, create_image_view::create_image_view};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_swapchain_image_views(device: &Device, data: &mut AppData) -> Result<()> {
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map(|i| create_image_view(device, *i, data.swapchain_format, vk::ImageAspectFlags::COLOR, 1))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    Ok(())
}
