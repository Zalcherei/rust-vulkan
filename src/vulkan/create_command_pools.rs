use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use super::{app_data::AppData, create_command_pool::create_command_pool};

pub unsafe fn create_command_pools(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Global

    data.command_pool = create_command_pool(instance, device, data).unwrap();

    // Per-framebuffer

    let num_images = data.swapchain_images.len();
    for _ in 0..num_images {
        let command_pool = create_command_pool(instance, device, data).unwrap();
        data.command_pools.push(command_pool);
    }

    Ok(())
}
