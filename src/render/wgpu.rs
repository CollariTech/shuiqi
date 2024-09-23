use async_trait::async_trait;
use wgpu::{Buffer, Device, DeviceDescriptor, IndexFormat, Instance, InstanceDescriptor, Queue, RenderPipeline, Surface, SurfaceConfiguration, TextureViewDescriptor};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::graphics::instance::{InstanceData, ObjectInstance, Shape, ShapeData};
use crate::render::Renderer;

pub struct WgpuRenderer<'window> {
    device: Device,
    queue: Queue,
    pub size: PhysicalSize<u32>,
    surface: Surface<'window>,
    config: SurfaceConfiguration,
    window: &'window Window,
    render_pipeline: RenderPipeline,
    instances: Vec<ObjectInstance>,
    instance_buffer: Buffer
}

impl<'window> WgpuRenderer<'window> {
    pub fn add_instance(&mut self, shape: ShapeData, position: [f32; 2], scale: [f32; 2]) {
        let instance_data = InstanceData::new(position, scale);
        self.instances.push(ObjectInstance::new(shape, instance_data));
        println!("Added instance: position: {:?}, scale: {:?}", position, scale);
        self.update_instance_buffer();
        println!("Total instances: {}", self.instances.len());
    }

    pub fn update_instance_buffer(&mut self) {
        let instance_data: Vec<_> = self.instances.iter().map(|i| i.data).collect();
        let buffer_size = instance_data.len() as u64 * std::mem::size_of::<InstanceData>() as u64;

        println!("Updating instance buffer with {} instances", instance_data.len());
        if self.instance_buffer.size() < buffer_size {
            println!("Resizing instance buffer to {}", buffer_size);
            self.instance_buffer = self.device.create_buffer_init(
                &BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                },
            );
        } else {
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));
        }
    }

    pub fn create_shape(&self, shape: Shape) -> ShapeData {
        let vertex_buffer = self.device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&shape.vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&shape.indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        ShapeData {
            vertex_buffer,
            index_buffer,
            indices_count: shape.indices.len() as u32
        }
    }
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

        let pipeline = crate::graphics::pipeline::create_instance_pipeline(
            &device
        );

        let instance_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            },
        );

        WgpuRenderer {
            device,
            queue,
            surface,
            window,
            config,
            size,
            render_pipeline: pipeline,
            instances: vec![],
            instance_buffer
        }
    }

    fn render(&self) {
        println!("Rendering with WGPU");
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(
            &TextureViewDescriptor::default()
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor::default()
        );
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0117647059,
                            g: 0.7890625,
                            b: 0.984375,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // Set the instance buffer for all instances at once
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            // Draw all instances in a single call
            if let Some(first_instance) = self.instances.first() {
                render_pass.set_vertex_buffer(0, first_instance.shape.vertex_buffer.slice(..));
                render_pass.set_index_buffer(first_instance.shape.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..first_instance.shape.indices_count, 0, 0..self.instances.len() as u32);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        println!("Resizing WGPU renderer to {}x{}", size.width, size.height);
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}