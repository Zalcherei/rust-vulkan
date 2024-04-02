#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod vulkan;

use crate::vulkan::App;
use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("Vulkan Tutorial (rust)").with_inner_size(LogicalSize::new(1024, 768)).build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;

    event_loop.run(move |event, control_flow| {
        match event {
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::RedrawRequested if !control_flow.exiting() && !minimized => {
                    unsafe { app.render(&window) }.unwrap();
                },
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.resized = true;
                    }
                }
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                    unsafe { app.destroy(); }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::ArrowLeft) if app.models > 1 => app.models -= 1,
                            PhysicalKey::Code(KeyCode::ArrowRight) if app.models < 4 => app.models += 1,
                            _ => { }
                        }
                    }
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}
