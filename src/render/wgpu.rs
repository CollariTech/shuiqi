use async_trait::async_trait;
use wgpu::{Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, Surface, SurfaceConfiguration, TextureViewDescriptor};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::render::Renderer;

pub struct WgpuRenderer<'window> {
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    surface: Surface<'window>,
    config: SurfaceConfiguration,
    window: &'window Window
}

#[async_trait(?Send)]
impl<'window> Renderer<'window> for WgpuRenderer<'window> {
    async fn init(window: &'window Window) -> WgpuRenderer<'window> {
        println!("Initializing WGPU renderer");
        let size = window.inner_size();

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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|format| format.is_srgb())
            .copied()
            .unwrap_or_else(|| surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 1,
            alpha_mode: Default::default(),
            view_formats: Default::default(),
        };
        surface.configure(&device, &config);

        println!("Device created: {:?}", device);

        WgpuRenderer {
            device,
            queue,
            surface,
            window,
            config,
            size
        }
    }

    async fn render(&self) {
        println!("Rendering with WGPU");
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(
            &TextureViewDescriptor::default()
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor::default()
        );
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    async fn resize(&mut self, size: PhysicalSize<u32>) {
        println!("Resizing WGPU renderer to {}x{}", size.width, size.height);
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}