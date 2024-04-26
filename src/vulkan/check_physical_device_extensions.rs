use super::{constants::DEVICE_EXTENSIONS, structures::SuitabilityError};
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use vulkanalia::prelude::v1_0::*;

pub unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)
        .unwrap()
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();
    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError("Missing required device extensions.")))
    }
}
