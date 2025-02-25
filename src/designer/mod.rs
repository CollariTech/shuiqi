pub mod point;

use crate::designer::point::{Measurement, Point};
use crate::graphics::instance::Shape;
use crate::graphics::Vertex;
use crate::render::Renderer;
use crate::render::wgpu::WgpuRenderer;

pub struct Designer;

impl Designer {
    pub fn new() -> Self {
        Self
    }

    pub fn create_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        center: Point,
        width: Measurement,
        height: Measurement,
        color: [f32; 3]
    ) {
        let [center_x, center_y] = center.to_ndc(renderer.size);
        let width_pixels = width.to_size(renderer.size.width);
        let height_pixels = height.to_size(renderer.size.height);
        let width_ndc = width_pixels / renderer.size.width as f32 * 2.0;
        let height_ndc = height_pixels / renderer.size.height as f32 * 2.0;
        let half_width = width_ndc / 2.0;
        let half_height = height_ndc / 2.0;

        let vertices = vec![
            Vertex::new([center_x - half_width, center_y + half_height], color),
            Vertex::new([center_x + half_width, center_y + half_height], color),
            Vertex::new([center_x - half_width, center_y - half_height], color),
            Vertex::new([center_x + half_width, center_y - half_height], color),
        ];

        let indices = vec![
            0, 2, 1,
            1, 2, 3
        ];

        let shape = renderer.create_shape(Shape { vertices, indices });
        renderer.add_instance(shape, [0.0, 0.0], [1.0, 1.0])
    }

    pub fn create_anchored_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        top_left: Point,
        width: Measurement,
        height: Measurement,
        color: [f32; 3]
    ) {
        let [x, y] = top_left.to_screen_space(renderer.size);
        let width_px = width.to_size(renderer.size.width);
        let height_px = height.to_size(renderer.size.height);
        let center_x = x + width_px / 2.0;
        let center_y = y + height_px / 2.0;
        let center = Point::from_pixels(center_x, center_y);
        self.create_rectangle(renderer, center, width, height, color)
    }
}
