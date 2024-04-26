use super::{
    app_data::AppData, begin_single_time_commands::begin_single_time_commands,
    end_single_time_commands::end_single_time_commands,
};
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn copy_buffer(
    device: &Device,
    data: &AppData,
    source: vk::Buffer,
    destination: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data).unwrap();

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, data, command_buffer).unwrap();

    Ok(())
}
