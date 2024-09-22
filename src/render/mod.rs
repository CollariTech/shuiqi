use async_trait::async_trait;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub mod wgpu;

#[async_trait(?Send)]
pub trait Renderer<'window> {
    async fn init(window: &'window Window) -> Self;

    fn render(&self);

    fn resize(&mut self, size: PhysicalSize<u32>);
}