pub mod config;
pub mod painter;
pub mod render;
pub mod shaders;
pub mod bus;
mod designer;

use std::time::Duration;
use tokio::task::JoinHandle;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopProxy;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::bus::ShuiqiEvent;
use crate::config::ShuiqiOptions;
use crate::render::Renderer;
use crate::render::wgpu::WgpuRenderer;

struct ShuiqiAppState {
    config: ShuiqiOptions,
    window: Option<Window>,
    renderer: Option<WgpuRenderer<'static>>,
    allow_resize: bool,
    resize_task: Option<JoinHandle<()>>,
    event_handler: Option<Box<dyn Fn(&mut WgpuRenderer<'static>, ShuiqiEvent) + Send + 'static>>,
    event_loop_proxy: EventLoopProxy<ShuiqiEvent>
}

pub struct ShuiqiApp {
    state: ShuiqiAppState,
    event_loop: EventLoop<ShuiqiEvent>,
}

impl ShuiqiApp {
    pub fn create() -> Self {
        let event_loop = EventLoop::<ShuiqiEvent>::with_user_event()
            .build()
            .expect("Failed to create event loop");
        let options = ShuiqiOptions::default();
        Self::new(options, event_loop)
    }

    pub fn new(options: ShuiqiOptions, event_loop: EventLoop<ShuiqiEvent>) -> Self {
        let event_loop_proxy = event_loop.create_proxy();
        ShuiqiApp {
            state: ShuiqiAppState {
                config: options,
                window: None,
                renderer: None,
                resize_task: None,
                allow_resize: true,
                event_handler: None,
                event_loop_proxy
            },
            event_loop,
        }
    }

    pub fn set_event_handler<F>(&mut self, callback: F)
    where
        F: Fn(&mut WgpuRenderer<'static>, ShuiqiEvent) + Send + 'static,
    {
        self.state.event_handler = Some(Box::new(callback));
    }

    pub fn start(self) {
        let state = self.state;
        let mut handler = ShuiqiHandler { state };

        self.event_loop.run_app(&mut handler).expect("Event loop failed");
    }
}

struct ShuiqiHandler {
    state: ShuiqiAppState
}

impl ApplicationHandler<ShuiqiEvent> for ShuiqiHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();

        futures::executor::block_on(async {
            let static_window = unsafe { std::mem::transmute::<&Window, &'static Window>(&window) };
            let renderer = WgpuRenderer::init(static_window).await;
            self.state.renderer = Some(renderer);
        });
        self.state.window = Some(window);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ShuiqiEvent) {
        match event {
            ShuiqiEvent::PerformResize(new_size) => {
                self.handle_resize(new_size);
            },
            _ => {}
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        self.state.event_loop_proxy.send_event(
            ShuiqiEvent::Window(event.clone())
        ).expect("Failed to send window event");
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing app");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(eventbus)) = (
                    self.state.renderer.as_mut(),
                    self.state.event_handler.as_ref(),
                ) {
                    if self.state.resize_task.is_none() {
                        eventbus(renderer, ShuiqiEvent::Redraw);
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if self.state.allow_resize {
                    self.schedule_resize(size);
                }
            }
            _ => {}
        }
    }
}

impl ShuiqiHandler {
    fn schedule_resize(&mut self, size: PhysicalSize<u32>) {
        if let Some(task) = self.state.resize_task.take() {
            if self.state.config.resize_interval_accumulates {
                return
            }
            task.abort();
        }

        let delay = self.state.config.resize_interval;
        let proxy = self.state.event_loop_proxy.clone();

        self.state.resize_task = Some(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay as u64)).await;
            let _ = proxy.send_event(ShuiqiEvent::PerformResize(size));
        }));
    }

    fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        if let (Some(renderer), Some(eventbus)) = (
            self.state.renderer.as_mut(),
            self.state.event_handler.as_ref(),
        ) {
            renderer.resize(size);
            eventbus(renderer, ShuiqiEvent::PerformResize(size));
            renderer.render();
        }
    }
}
