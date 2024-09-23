mod render;
mod graphics;
mod config;
mod designer;

use crate::config::ShuiqiOptions;
use crate::designer::Designer;
use crate::render::wgpu::WgpuRenderer;
use crate::render::Renderer;
use futures::FutureExt;
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::designer::point::{Measurement, Point};

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

            let designer = Designer::new();
            designer.create_rectangle(
                &mut renderer,
                Point::new(Measurement::Percentage(0.2), Measurement::Percentage(0.2)),
                Point::new(Measurement::Percentage(0.5), Measurement::Percentage(0.5))
            );

            renderer.resize(size);
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


    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
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
                        let renderer = clone.lock().await;
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
    let mut app = ShuqiApp::default();
    let mut intermediate = ShuqiIntermediateApp::new(app);
    intermediate.start();
}