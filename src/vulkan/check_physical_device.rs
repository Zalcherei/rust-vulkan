use super::{
    app_data::AppData,
    check_physical_device_extensions::check_physical_device_extensions,
    structures::{QueueFamilyIndices, SuitabilityError, SwapchainSupport},
};
use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;

pub unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device).unwrap();
    check_physical_device_extensions(instance, physical_device).unwrap();

    let support = SwapchainSupport::get(instance, data, physical_device).unwrap();
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.sampler_anisotropy != vk::TRUE {
        return Err(anyhow!(SuitabilityError("No sampler anisotropy.")));
    }

    Ok(())
}
