pub mod point;

use crate::designer::point::Point;
use crate::graphics::instance::Shape;
use crate::graphics::Vertex;
use crate::render::wgpu::WgpuRenderer;

pub struct Designer;

impl Designer {
    pub fn new() -> Self {
        Self
    }

    pub fn create_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        first: Point,
        second: Point
    ) {
        let vertices = vec![
            Vertex::new([-1.0, 1.0], [0.5, 0.5, 0.5]), // top left
            Vertex::new([1.0, 0.9], [0.5, 0.5, 0.5]), // bottom right
            Vertex::new([-1.0, 0.9], [0.5, 0.5, 0.5]), // bottom left
            Vertex::new([1.0, 1.0], [0.5, 0.5, 0.5]) // top right
        ];

        let indices = vec![
            1, 3, 0,
            0, 2, 1
        ];
        let shape = renderer.create_shape(
            Shape {
                vertices,
                indices
            }
        );
        renderer.add_instance(
            shape,
            [0.0, 0.0],
            [1.0, 1.0]
        )
    }
}