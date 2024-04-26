use super::{
    app_data::AppData, create_image::create_image, create_image_view::create_image_view,
    get_depth_format::get_depth_format,
};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn create_depth_objects(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Image + Image Memory

    let format = get_depth_format(instance, data).unwrap();

    let (depth_image, depth_image_memory) = create_image(
        instance,
        device,
        data,
        data.swapchain_extent.width,
        data.swapchain_extent.height,
        1,
        data.msaa_samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .unwrap();

    data.depth_image = depth_image;
    data.depth_image_memory = depth_image_memory;

    // Image View

    data.depth_image_view =
        create_image_view(device, data.depth_image, format, vk::ImageAspectFlags::DEPTH, 1).unwrap();

    Ok(())
}
