use super::app_data::AppData;
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn end_single_time_commands(
    device: &Device,
    data: &AppData,
    command_buffer: vk::CommandBuffer,
) -> Result<()> {
    // End

    device.end_command_buffer(command_buffer).unwrap();

    // Submit

    let command_buffers = &[command_buffer];
    let info = vk::SubmitInfo::builder().command_buffers(command_buffers);

    device
        .queue_submit(data.graphics_queue, &[info], vk::Fence::null())
        .unwrap();
    device.queue_wait_idle(data.graphics_queue).unwrap();

    // Cleanup

    device.free_command_buffers(data.command_pool, &[command_buffer]);

    Ok(())
}
