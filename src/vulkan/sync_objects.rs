use super::{constants::MAX_FRAMES_IN_FLIGHT, structures::AppData};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

//================================================
// Sync Objects
//================================================

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None).unwrap());
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None).unwrap());

        data.in_flight_fences
            .push(device.create_fence(&fence_info, None).unwrap());
    }

    data.images_in_flight = data.swapchain_images.iter().map(|_| vk::Fence::null()).collect();

    Ok(())
}
