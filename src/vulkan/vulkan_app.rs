use crate::vulkan::{
    create_color_objects, create_command_buffers, create_command_pool, create_depth_objects, create_descriptor_pool,
    create_descriptor_set_layout, create_descriptor_sets, create_framebuffers, create_index_buffer, create_instance,
    create_logical_device, create_pipeline, create_render_pass, create_swapchain, create_swapchain_image_views,
    create_sync_objects, create_texture_image, create_texture_image_view, create_texture_sampler,
    create_uniform_buffers, create_vertex_buffer, load_model, pick_physical_device, AppData, UniformBufferObject,
};
use anyhow::{anyhow, Result};
use nalgebra_glm as glm;
use std::{mem::size_of, ptr::copy_nonoverlapping as memcpy, time::Instant};
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    prelude::v1_0::*,
    vk::ExtDebugUtilsExtension,
    vk::KhrSurfaceExtension,
    vk::KhrSwapchainExtension,
};
use winit::window::Window;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[derive(Clone, Debug)]
pub struct App {
    pub entry: Entry,
    pub instance: Instance,
    pub data: AppData,
    pub device: Device,
    pub frame: usize,
    pub resized: bool,
    pub start: Instant,
    pub models: usize,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        create_color_objects(&instance, &device, &mut data)?;
        create_depth_objects(&instance, &device, &mut data)?;
        create_texture_image(&instance, &device, &mut data)?;
        create_texture_image_view(&device, &mut data)?;
        create_texture_sampler(&device, &mut data)?;
        load_model(&mut data)?;
        create_vertex_buffer(&instance, &device, &mut data)?;
        create_index_buffer(&instance, &device, &mut data)?;
        create_uniform_buffers(&instance, &device, &mut data)?;
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
            frame: 0,
            resized: false,
            start: Instant::now(),
            models: 1,
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::max_value())?;

        let image_index = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain,
                u64::max_value(),
                self.data.image_available_semaphores[self.frame],
                vk::Fence::null(),
            )?
            .0 as usize;

        let image_in_flight = self.data.images_in_flight[image_index];
        if !image_in_flight.is_null() {
            self.device
                .wait_for_fences(&[image_in_flight], true, u64::max_value())?;
        }

        self.data.images_in_flight[image_index] = in_flight_fence;

        self.update_command_buffer(image_index)?;
        self.update_uniform_buffer(image_index)?;

        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index]];
        let signal_semaphores = &[self.data.render_finished_semaphores[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.reset_fences(&[in_flight_fence])?;

        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(wait_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let result = self.device.queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);
        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain(window)?;
        } else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

  #[rustfmt::skip]
    pub unsafe fn update_command_buffer(&mut self, image_index: usize) -> Result<()> {
      let command_pool = self.data.command_pools[image_index];
      self.device
          .reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;

      let allocate_info = vk::CommandBufferAllocateInfo::builder()
          .command_pool(command_pool)
          .level(vk::CommandBufferLevel::PRIMARY)
          .command_buffer_count(1);

      let command_buffer = self.device.allocate_command_buffers(&allocate_info)?[0];
      self.data.command_buffers[image_index] = command_buffer;

      let time = self.start.elapsed().as_secs_f32();

      let model = glm::rotate(
          &glm::identity(),
          time * glm::radians(&glm::vec1(90.0))[0],
          &glm::vec3(0.0, 0.0, 1.0),
      );

      let (_, model_bytes, _) = model.as_slice().align_to::<u8>();

      let opacity = 0.25f32;
      let opacity_bytes = &opacity.to_ne_bytes()[..];

      let info = vk::CommandBufferBeginInfo::builder();

      self.device.begin_command_buffer(command_buffer, &info)?;

      let render_area = vk::Rect2D::builder()
          .offset(vk::Offset2D::default())
          .extent(self.data.swapchain_extent);

      let color_clear_value = vk::ClearValue {
          color: vk::ClearColorValue {
              float32: [0.0, 0.0, 0.0, 1.0],
          },
      };

      let depth_clear_value = vk::ClearValue {
          depth_stencil: vk::ClearDepthStencilValue {
              depth: 1.0,
              stencil: 0,
          },
      };

      let clear_values = &[color_clear_value, depth_clear_value];
      let info = vk::RenderPassBeginInfo::builder()
          .render_pass(self.data.render_pass)
          .framebuffer(self.data.framebuffers[image_index])
          .render_area(render_area)
          .clear_values(clear_values);

      self.device
          .cmd_begin_render_pass(command_buffer, &info, vk::SubpassContents::INLINE);
      self.device.cmd_bind_pipeline(
          command_buffer,
          vk::PipelineBindPoint::GRAPHICS,
          self.data.pipeline,
      );
      self.device
          .cmd_bind_vertex_buffers(command_buffer, 0, &[self.data.vertex_buffer], &[0]);
      self.device.cmd_bind_index_buffer(
          command_buffer,
          self.data.index_buffer,
          0,
          vk::IndexType::UINT32,
      );
      self.device.cmd_bind_descriptor_sets(
          command_buffer,
          vk::PipelineBindPoint::GRAPHICS,
          self.data.pipeline_layout,
          0,
          &[self.data.descriptor_sets[image_index]],
          &[],
      );
      self.device.cmd_push_constants(
          command_buffer,
          self.data.pipeline_layout,
          vk::ShaderStageFlags::VERTEX,
          0,
          model_bytes,
      );
      self.device.cmd_push_constants(
          command_buffer,
          self.data.pipeline_layout,
          vk::ShaderStageFlags::FRAGMENT,
          64,
          opacity_bytes,
      );
      self.device
          .cmd_draw_indexed(command_buffer, self.data.indices.len() as u32, 1, 0, 0, 0);
      self.device.cmd_end_render_pass(command_buffer);
      self.device.end_command_buffer(command_buffer)?;

      Ok(())
  }

  #[rustfmt::skip]
    pub unsafe fn update_secondary_command_buffer(&mut self, image_index: usize, model_index: usize) -> Result<vk::CommandBuffer> {
      let allocate_info = vk::CommandBufferAllocateInfo::builder().command_pool(self.data.command_pools[image_index]).level(vk::CommandBufferLevel::SECONDARY).command_buffer_count(1);

      let command_buffer = self.device.allocate_command_buffers(&allocate_info)?[0];

      let y = (((model_index % 2) as f32) * 2.5) - 1.25;
      let z = (((model_index / 2) as f32) * -2.0) + 1.0;

      let model = glm::translate(&glm::identity(), &glm::vec3(0.0, y, z));

      let time = self.start.elapsed().as_secs_f32();

      let model = glm::rotate(&model, time * glm::radians(&glm::vec1(90.0))[0], &glm::vec3(0.0, 0.0, 1.0));

      let (_, model_bytes, _) = model.as_slice().align_to::<u8>();

      let opacity = (model_index + 1) as f32 * 0.25;
      let opacity_bytes = &opacity.to_ne_bytes()[..];

      let inheritance_info = vk::CommandBufferInheritanceInfo::builder().render_pass(self.data.render_pass).subpass(0).framebuffer(self.data.framebuffers[image_index]);

      let info = vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE).inheritance_info(&inheritance_info);

      self.device.begin_command_buffer(command_buffer, &info)?;

      self.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.data.pipeline);
      self.device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.data.vertex_buffer], &[0]);
      self.device.cmd_bind_index_buffer(command_buffer, self.data.index_buffer, 0, vk::IndexType::UINT32);
      self.device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.data.pipeline_layout, 0, &[self.data.descriptor_sets[image_index]], &[]);
      self.device.cmd_push_constants(command_buffer, self.data.pipeline_layout, vk::ShaderStageFlags::VERTEX, 0, model_bytes);
      self.device.cmd_push_constants(command_buffer, self.data.pipeline_layout, vk::ShaderStageFlags::FRAGMENT, 64, opacity_bytes);
      self.device.cmd_draw_indexed(command_buffer, self.data.indices.len() as u32, 1, 0, 0, 0);
      self.device.end_command_buffer(command_buffer)?;

      Ok(command_buffer)
  }

    pub unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {
        let time = self.start.elapsed().as_secs_f32();

        let model = glm::rotate(
            &glm::identity(),
            time * glm::radians(&glm::vec1(90.0))[0],
            &glm::vec3(0.0, 0.0, 1.0),
        );

        let view = glm::look_at(
            &glm::vec3(2.0, 2.0, 2.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 0.0, 1.0),
        );

        let mut proj = glm::perspective(
            self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
            glm::radians(&glm::vec1(45.0))[0],
            0.1,
            10.0,
        );

        proj[(1, 1)] *= -1.0;

        let ubo = UniformBufferObject { model, view, proj };

        let memory = self.device.map_memory(
            self.data.uniform_buffers_memory[image_index],
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device.unmap_memory(self.data.uniform_buffers_memory[image_index]);

        Ok(())
    }

  #[rustfmt::skip]
    pub unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
      self.device.device_wait_idle()?;
      self.destroy_swapchain();
      create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
      create_swapchain_image_views(&self.device, &mut self.data)?;
      create_render_pass(&self.instance, &self.device, &mut self.data)?;
      create_pipeline(&self.device, &mut self.data)?;
      create_color_objects(&self.instance, &self.device, &mut self.data)?;
      create_depth_objects(&self.instance, &self.device, &mut self.data)?;
      create_framebuffers(&self.device, &mut self.data)?;
      create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
      create_descriptor_pool(&self.device, &mut self.data)?;
      create_descriptor_sets(&self.device, &mut self.data)?;
      create_command_buffers(&self.device, &mut self.data)?;
      self.data
          .images_in_flight
          .resize(self.data.swapchain_images.len(), vk::Fence::null());

      Ok(())
  }

  #[rustfmt::skip]
    pub unsafe fn destroy(&mut self) {
      self.device.device_wait_idle().unwrap();

      self.destroy_swapchain();

      self.data
          .in_flight_fences
          .iter()
          .for_each(|f| self.device.destroy_fence(*f, None));
      self.data
          .render_finished_semaphores
          .iter()
          .for_each(|s| self.device.destroy_semaphore(*s, None));
      self.data
          .image_available_semaphores
          .iter()
          .for_each(|s| self.device.destroy_semaphore(*s, None));
      self.device.free_memory(self.data.index_buffer_memory, None);
      self.device.destroy_buffer(self.data.index_buffer, None);
      self.device
          .free_memory(self.data.vertex_buffer_memory, None);
      self.device.destroy_buffer(self.data.vertex_buffer, None);
      self.device.destroy_sampler(self.data.texture_sampler, None);
      self.device
          .destroy_image_view(self.data.texture_image_view, None);
      self.device
          .free_memory(self.data.texture_image_memory, None);
      self.device.destroy_image(self.data.texture_image, None);
      self.device
          .destroy_command_pool(self.data.command_pool, None);
      self.device
          .destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);
      self.device.destroy_device(None);
      self.instance.destroy_surface_khr(self.data.surface, None);

      if VALIDATION_ENABLED {
          self.instance
              .destroy_debug_utils_messenger_ext(self.data.messenger, None);
      }

      self.instance.destroy_instance(None);
  }

  #[rustfmt::skip]
    pub unsafe fn destroy_swapchain(&mut self) {
      self.device
          .free_command_buffers(self.data.command_pool, &self.data.command_buffers);
      self.device
          .destroy_descriptor_pool(self.data.descriptor_pool, None);
      self.data
          .uniform_buffers_memory
          .iter()
          .for_each(|m| self.device.free_memory(*m, None));
      self.data
          .uniform_buffers
          .iter()
          .for_each(|b| self.device.destroy_buffer(*b, None));
      self.data
          .framebuffers
          .iter()
          .for_each(|f| self.device.destroy_framebuffer(*f, None));
      self.device.destroy_pipeline(self.data.pipeline, None);
      self.device
          .destroy_pipeline_layout(self.data.pipeline_layout, None);
      self.device.destroy_render_pass(self.data.render_pass, None);
      self.data
          .swapchain_image_views
          .iter()
          .for_each(|v| self.device.destroy_image_view(*v, None));
      self.device.destroy_swapchain_khr(self.data.swapchain, None);
  }
}
