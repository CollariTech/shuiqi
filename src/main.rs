mod render;
mod graphics;

use std::sync::Arc;
use std::time::Instant;
use futures::FutureExt;
use tokio::sync::Mutex;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::render::Renderer;
use crate::render::wgpu::WgpuRenderer;

#[derive(Default)]
struct ShuiqiApp {
    // This is here only to keep the window alive
    window: Option<Window>,
    renderer: Option<Arc<Mutex<WgpuRenderer<'static>>>>,
    last_resize: Option<Instant>
}

impl ShuiqiApp {
    fn new() -> Self {
        ShuiqiApp {
            window: None,
            renderer: None,
            last_resize: None
        }
    }

    fn start(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let res = event_loop.run_app(self);
        match res {
            Ok(_) => println!("App exited successfully"),
            Err(e) => println!("App exited with error: {}", e)
        }
    }
}

impl ApplicationHandler for ShuiqiApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();

        futures::executor::block_on(async {
            let static_window = unsafe {
                std::mem::transmute::<&Window, &'static Window>(&window)
            };
            self.renderer = Some(Arc::new(Mutex::new(WgpuRenderer::init(static_window).await)));
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
                if let Some(renderer) = &self.renderer {
                    let clone = Arc::clone(renderer);
                    tokio::spawn(async move {
                        let renderer = clone.lock().await;
                        renderer.render();
                    });
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &self.renderer {
                    let clone = Arc::clone(renderer);
                    tokio::spawn(async move {
                        let mut renderer = clone.lock().await;
                        renderer.resize(size);
                    });
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let mut app = ShuiqiApp::new();
    app.start();
}