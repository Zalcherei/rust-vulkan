use super::{
    shared_buffers::{copy_buffer, create_buffer},
    structures::{AppData, UniformBufferObject, Vertex},
};
use anyhow::Result;
use std::{mem::size_of, ptr::copy_nonoverlapping as memcpy};
use vulkanalia::prelude::v1_0::*;

//================================================
// Buffers
//================================================

pub unsafe fn create_vertex_buffer(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Create (staging)

    let size = (size_of::<Vertex>() * data.vertices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )
    .unwrap();

    // Copy (staging)

    let memory = device
        .map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())
        .unwrap();

    memcpy(data.vertices.as_ptr(), memory.cast(), data.vertices.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (vertex)

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .unwrap();

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    // Copy (vertex)

    copy_buffer(device, data, staging_buffer, vertex_buffer, size).unwrap();

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub unsafe fn create_index_buffer(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Create (staging)

    let size = (size_of::<u32>() * data.indices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )
    .unwrap();

    // Copy (staging)

    let memory = device
        .map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())
        .unwrap();

    memcpy(data.indices.as_ptr(), memory.cast(), data.indices.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (index)

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .unwrap();

    data.index_buffer = index_buffer;
    data.index_buffer_memory = index_buffer_memory;

    // Copy (index)

    copy_buffer(device, data, staging_buffer, index_buffer, size).unwrap();

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub unsafe fn create_uniform_buffers(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )
        .unwrap();

        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}
