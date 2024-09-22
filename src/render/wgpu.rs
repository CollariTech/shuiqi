use async_trait::async_trait;
use wgpu::{Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, Surface};
use winit::window::Window;
use crate::render::Renderer;

pub struct WgpuRenderer<'window> {
    device: Device,
    queue: Queue,
    surface: Surface<'window>,
    window: &'window Window
}

#[async_trait(?Send)]
impl<'window> Renderer<'window> for WgpuRenderer<'window> {
    async fn init(window: &'window Window) -> WgpuRenderer<'window> {
        println!("Initializing WGPU renderer");
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor::default(),
            None
        ).await.unwrap();

        println!("Device created: {:?}", device);

        WgpuRenderer {
            device,
            queue,
            surface,
            window
        }
    }

    async fn render(&self) {
        println!("Rendering with WGPU");
    }
}