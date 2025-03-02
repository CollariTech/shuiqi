use crate::render::shape::{ObjectInstance, Shape, ShapeData, TextInstance};
use crate::render::Renderer;
use crate::shaders::pipeline::create_instance_pipeline;
use crate::shaders::InstanceData;
use async_trait::async_trait;
use glyphon::{FontSystem, SwashCache, TextArea, TextAtlas, TextRenderer};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, Device, DeviceDescriptor, IndexFormat, Instance, InstanceDescriptor, MultisampleState, Queue, RenderPipeline, Surface, SurfaceConfiguration, TextureViewDescriptor};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WgpuRenderer<'window> {
    device: Device,
    queue: Queue,
    pub size: PhysicalSize<u32>,
    surface: Surface<'window>,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    instances: Vec<ObjectInstance>,
    instance_buffer: Buffer,
    pub font_system: FontSystem,
    swash_cache: SwashCache,
    text_renderer: TextRenderer,
    text_areas: Vec<TextArea<'static>>,
    text_atlas: TextAtlas,
    viewport: glyphon::Viewport,
    next_shape_id: u32
}

impl<'window> WgpuRenderer<'window> {
    pub fn add_instance(&mut self, shape: ShapeData, position: [f32; 2], scale: [f32; 2]) {
        let instance_data = InstanceData::new(position, scale);
        self.instances.push(ObjectInstance::new(shape, instance_data));
        self.update_instance_buffer();
    }

    pub fn update_instance_buffer(&mut self) {
        self.instances.sort_by_key(|instance| instance.shape.shape_id);
        let instance_data: Vec<_> = self.instances.iter().map(|i| i.data).collect();

        let buffer_size = (instance_data.len() * std::mem::size_of::<InstanceData>()) as u64;
        if self.instance_buffer.size() < buffer_size {
            self.instance_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));
        }
    }

    pub fn create_shape(&mut self, shape: Shape) -> ShapeData {
        let shape_id = self.next_shape_id;
        self.next_shape_id += 1;

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
            shape_id,
            vertex_buffer,
            index_buffer,
            indices_count: shape.indices.len() as u32
        }
    }

    pub fn add_text<'a>(&mut self, text: TextInstance) {
        let text_buffer = Box::leak(Box::new(glyphon::Buffer::new(
            &mut self.font_system,
            glyphon::Metrics::new(text.font_size, text.line_height)
        )));
        text_buffer.set_text(
            &mut self.font_system,
            &text.content,
            glyphon::Attrs::new().family(text.font_family),
            glyphon::Shaping::Advanced
        );
        println!("Adding text to position ({}, {})", text.text_area.left, text.text_area.top);
        self.text_areas.push(TextArea {
            buffer: text_buffer,
            left: text.text_area.left,
            top: text.text_area.top,
            scale: text.text_area.scale,
            bounds: text.text_area.bounds,
            default_color: text.text_area.color.into(),
            custom_glyphs: &[]
        });
    }

    pub fn measure_text_size(
        &mut self,
        content: &str,
        font_family: glyphon::Family<'static>,
        font_size: f32,
        line_height: f32,
    ) -> (f32, f32) {
        let mut temp_buffer = glyphon::Buffer::new(
            &mut self.font_system,
            glyphon::Metrics::new(font_size, line_height),
        );
        temp_buffer.set_size(&mut self.font_system, None, None);
        temp_buffer.set_text(
            &mut self.font_system,
            &content,
            glyphon::Attrs::new().family(font_family),
            glyphon::Shaping::Advanced,
        );
        let runs: Vec<_> = temp_buffer.layout_runs().collect();
        if runs.is_empty() {
            (0.0, 0.0)
        } else {
            let max_width = runs.iter().map(|run| run.line_w).fold(0.0, f32::max);
            let last_run = runs.last().unwrap();
            let total_height = last_run.line_y + last_run.line_height;
            (max_width, total_height)
        }
    }
}

#[async_trait(?Send)]
impl<'window> Renderer<'window> for WgpuRenderer<'window> {
    async fn init(window: &'window Window) -> WgpuRenderer<'window> {
        let size = window.inner_size();

        let instance = Instance::new(&InstanceDescriptor::default());
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
            .unwrap_or_else(|| surface_caps.formats[0])
            .add_srgb_suffix();

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

        let pipeline = create_instance_pipeline(
            &device,
            surface_format
        );

        let instance_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            },
        );

        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = glyphon::Cache::new(&device);
        let viewport = glyphon::Viewport::new(&device, &cache);
        let mut text_atlas = TextAtlas::new(&device, &queue, &cache, surface_format);

        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            MultisampleState::default(),
            None
        );

        WgpuRenderer {
            device,
            queue,
            surface,
            config,
            size,
            render_pipeline: pipeline,
            instances: Vec::new(),
            instance_buffer,
            text_renderer,
            font_system,
            swash_cache,
            text_areas: Vec::new(),
            text_atlas,
            viewport,
            next_shape_id: 0
        }
    }

    fn render(&mut self) {
        println!("Rendering with WGPU");
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(
            &TextureViewDescriptor::default()
        );
        self.viewport.update(&self.queue, glyphon::Resolution {
            width: self.size.width,
            height: self.size.height,
        });

        let text_areas = self
            .text_areas
            .iter()
            .map(|area| area.clone())
            .collect::<Vec<TextArea<'static>>>();
        match self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &mut self.viewport,
            text_areas,
            &mut self.swash_cache,
        ) {
            Ok(_) => println!("Text renderer prepared"),
            Err(e) => eprintln!("Error preparing text renderer: {:?}", e),
        }

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
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            let mut current_shape: Option<&ShapeData> = None;
            let mut start = 0;
            for (i, instance) in self.instances.iter().enumerate() {
                if current_shape != Some(&instance.shape) {
                    if let Some(shape) = current_shape {
                        render_pass.set_vertex_buffer(0, shape.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(shape.index_buffer.slice(..), IndexFormat::Uint16);
                        render_pass.draw_indexed(0..shape.indices_count, 0, start as u32..i as u32);
                    }
                    current_shape = Some(&instance.shape);
                    start = i;
                }
            }
            if let Some(shape) = current_shape {
                render_pass.set_vertex_buffer(0, shape.vertex_buffer.slice(..));
                render_pass.set_index_buffer(shape.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(
                    0..shape.indices_count,
                    0,
                    start as u32..self.instances.len() as u32,
                );
            }

            match self.text_renderer.render(
                &self.text_atlas,
                &self.viewport,
                &mut render_pass
            ) {
                Ok(_) => println!("Text renderer rendered"),
                Err(e) => eprintln!("Error rendering text: {:?}", e),
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

    fn reset(&mut self) {
        self.instances.clear();
        self.text_areas.clear()
    }
}

unsafe impl Send for WgpuRenderer<'_> {}
