use crate::painter::Object;
use crate::painter::point::Point;
use crate::render::wgpu::WgpuRenderer;

pub trait Element {
    fn render(
        &self,
        renderer: &mut WgpuRenderer,
        point: Point
    ) -> Object;
}