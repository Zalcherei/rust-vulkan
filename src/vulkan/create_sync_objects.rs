use crate::vulkan::AppData;
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores.push(device.create_semaphore(&semaphore_info, None)?);
        data.render_finished_semaphores.push(device.create_semaphore(&semaphore_info, None)?);

    }

    data.in_flight_fences = data.swapchain_images.iter().map(|_| vk::Fence::null()).collect();

    Ok(())
}
