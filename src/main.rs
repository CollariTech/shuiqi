mod render;
mod shaders;
mod config;
mod painter;

use crate::config::ShuiqiOptions;
use crate::painter::color::Color;
use crate::painter::point::{Measurement, Point};
use crate::painter::text::InnerText;
use crate::painter::writer::draw_objects;
use crate::painter::Object;
use crate::render::wgpu::WgpuRenderer;
use crate::render::Renderer;
use std::sync::Arc;
use tokio::sync::Mutex;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct ShuqiApp {
    config: ShuiqiOptions
}

#[derive(Default)]
struct ShuqiIntermediateApp {
    pub app: ShuqiApp,
    pub window: Option<Window>,
    pub renderer: Option<Arc<Mutex<WgpuRenderer<'static>>>>,
    pub allow_resize: bool,
    pub resize_task: Option<tokio::task::JoinHandle<()>>
}

impl ShuqiIntermediateApp {
    fn new(app: ShuqiApp) -> Self {
        ShuqiIntermediateApp {
            app,
            window: None,
            renderer: None,
            resize_task: None,
            allow_resize: true
        }
    }

    pub fn start(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let res = event_loop.run_app(self);

        match res {
            Ok(_) => println!("App exited successfully"),
            Err(e) => println!("App exited with error: {}", e)
        }
    }

    fn schedule_resize(&mut self, size: PhysicalSize<u32>) {
        if let Some(task) = self.resize_task.take() {
            task.abort();
        }

        let delay = self.app.config.resize_interval;
        let clone = Arc::clone(self.renderer.as_ref().unwrap());

        self.resize_task = Some(tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
            let mut renderer = clone.lock().await;
            renderer.reset();
            renderer.resize(size);

            let rectangle = Object::colored(
                Point::from_pixels(12.0, 12.0),
                Measurement::Pixels(50.0),
                Measurement::Pixels(50.0),
                Color::new(255, 0, 0)
            );

            // designer.create_text(
            //     &mut renderer,
            //     Point::from_percentage(50.0, 50.0),
            //     "Hello, world!",
            //     Family::Name("Roboto"),
            //     48.0,
            //     1.0,
            //     None,
            //     Color::new(0, 0, 0)
            // );

            let circle = Object::colored(
                Point::from_pixels(25.0, 300.0),
                Measurement::Pixels(50.0),
                Measurement::Pixels(50.0),
                Color::new(0, 255, 0)
            )
                .border_radius(Measurement::Percentage(50.0));

            let rounded_rectangle = Object::colored(
                Point::from_percentage(50.0, 50.0),
                Measurement::Pixels(120.0),
                Measurement::Pixels(60.0),
                Color::new(255, 0, 0)
            )
                .centered()
                .border_radius(Measurement::Pixels(36.0))
                .text(
                    InnerText::of("Hello, world!")
                        .centered()
                );

            draw_objects(
                &mut renderer,
                vec![rectangle, circle, rounded_rectangle]
            );

            renderer.render();
        }));
    }
}

impl ApplicationHandler for ShuqiIntermediateApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();

        futures::executor::block_on(async {
            let static_window = unsafe {
                std::mem::transmute::<&Window, &'static Window>(&window)
            };
            let renderer = WgpuRenderer::init(static_window).await;


            self.renderer = Some(Arc::new(Mutex::new(renderer)));
        });
        self.window = Some(window);
    }


    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing app");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if self.resize_task.is_some() {
                    return;
                }

                if let Some(renderer) = &self.renderer {
                    let clone = Arc::clone(renderer);
                    tokio::spawn(async move {
                        let mut renderer = clone.lock().await;
                        renderer.render();
                    });
                }
            }
            WindowEvent::Resized(size) => {
                self.schedule_resize(size);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let app = ShuqiApp::default();
    let mut intermediate = ShuqiIntermediateApp::new(app);
    intermediate.start();
}