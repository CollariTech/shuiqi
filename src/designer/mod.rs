pub mod point;

use crate::designer::point::{get_measurement_screen_percentage, Measurement, Point};
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
        first: Point,
        second: Point
    ) {
        let first_percentages = first.get_screen_position(renderer.size);
        let first_x = first_percentages[0];
        let first_y = first_percentages[1];

        let second_percentages = second.get_screen_position(renderer.size);
        let second_x = second_percentages[0];
        let second_y = second_percentages[1];

        let color = [0.5, 0.5, 0.5];
        let top_left = Vertex::new(
            [
                Self::min(first_x, second_x),
                Self::max(first_y, second_y)
            ], color
        );
        let top_right = Vertex::new(
            [
                Self::max(first_x, second_x),
                Self::max(first_y, second_y)
            ], color
        );
        let bottom_left = Vertex::new(
            [
                Self::min(first_x, second_x),
                Self::min(first_y, second_y)
            ], color
        );
        let bottom_right = Vertex::new(
            [
                Self::max(first_x, second_x),
                Self::min(first_y, second_y)
            ], color
        );

        let vertices = vec![
            top_left,
            bottom_right,
            bottom_left,
            top_right,
        ];
        println!("{:?}", vertices);

        let indices: Vec<u16> = vec![
            1, 3, 0,
            0, 2, 1
        ];
        println!("{:?}", indices);
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

    pub fn create_relative_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        point: Point,
        width: Measurement,
        height: Measurement
    ) {
        let point_measurements = point.get_screen_position(renderer.size);
        let x = point_measurements[0];
        let y = point_measurements[1];

        let width_percentage = get_measurement_screen_percentage(&width, renderer.size.width);
        let height_percentage = get_measurement_screen_percentage(&height, renderer.size.height);

        let color = [0.5, 0.5, 0.5];
        let top_left = Vertex::new(
            [
                x - width_percentage / 2.0,
                y + height_percentage / 2.0
            ], color
        );
        let top_right = Vertex::new(
            [
                x + width_percentage / 2.0,
                y + height_percentage / 2.0
            ], color
        );
        let bottom_left = Vertex::new(
            [
                x - width_percentage / 2.0,
                y - height_percentage / 2.0
            ], color
        );
        let bottom_right = Vertex::new(
            [
                x + width_percentage / 2.0,
                y - height_percentage / 2.0
            ], color
        );

        let vertices = vec![
            top_left,
            bottom_right,
            bottom_left,
            top_right,
        ];

        let indices: Vec<u16> = vec![
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
            point_measurements,
            [1.0, 1.0]
        )
    }

    fn get_counter_clockwise_index_order(vertexes: &Vec<Vertex>) -> Vec<u16> {
        if vertexes.is_empty() {
            return vec![];
        }

        let centroid = {
            let sum = vertexes.iter().fold([0.0, 0.0], |[sx, sy], v| {
                [sx + v.position[0], sy + v.position[1]]
            });
            let len = vertexes.len() as f32;
            [sum[0] / len, sum[1] / len]
        };

        let mut indices_with_angles: Vec<(usize, f32)> = vertexes
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let dx = v.position[0] - centroid[0];
                let dy = v.position[1] - centroid[1];
                let angle = dy.atan2(dx);
                (i, angle)
            })
            .collect();

        indices_with_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        indices_with_angles.into_iter().map(|(i, _)| i as u16).collect()
    }

    fn max(a: f32, b: f32) -> f32 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn min(a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }
}