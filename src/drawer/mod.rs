pub mod point;
pub mod color;

use glyphon::{Family, TextBounds};
use crate::drawer::color::Color;
use crate::drawer::point::{Measurement, Point};
use crate::render::shape::{Shape, TextInstance, TextInstanceArea};
use crate::render::wgpu::WgpuRenderer;
use crate::shaders::Vertex;

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
        color: Color
    ) {
        let [center_x, center_y] = center.to_ndc(renderer.size);
        let width_pixels = width.to_size(renderer.size.width);
        let height_pixels = height.to_size(renderer.size.height);
        let width_ndc = width_pixels / renderer.size.width as f32 * 2.0;
        let height_ndc = height_pixels / renderer.size.height as f32 * 2.0;
        let half_width = width_ndc / 2.0;
        let half_height = height_ndc / 2.0;
        let rgb = color.into();

        let vertices = vec![
            Vertex::new([center_x - half_width, center_y + half_height], rgb),
            Vertex::new([center_x + half_width, center_y + half_height], rgb),
            Vertex::new([center_x - half_width, center_y - half_height], rgb),
            Vertex::new([center_x + half_width, center_y - half_height], rgb),
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
        color: Color
    ) {
        let [x, y] = top_left.to_screen_space(renderer.size);
        let width_px = width.to_size(renderer.size.width);
        let height_px = height.to_size(renderer.size.height);
        let center_x = x + width_px / 2.0;
        let center_y = y + height_px / 2.0;
        let center = Point::from_pixels(center_x, center_y);
        self.create_rectangle(renderer, center, width, height, color)
    }

    // More performant than create_rounded_rectangle with a 50% border radius
    pub fn create_circle(
        &self,
        renderer: &mut WgpuRenderer,
        center: Point,
        radius: Measurement,
        color: Color,
        segments: u16,
    ) {
        let [center_x, center_y] = center.to_ndc(renderer.size);

        let radius_pixels = radius.to_size(renderer.size.width);

        let radius_ndc_x = radius_pixels * (2.0 / renderer.size.width as f32);
        let radius_ndc_y = radius_pixels * (2.0 / renderer.size.height as f32);

        let rgb = color.into();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.push(Vertex::new([center_x, center_y], rgb));

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = center_x + radius_ndc_x * angle.cos();
            let y = center_y + radius_ndc_y * angle.sin();
            vertices.push(Vertex::new([x, y], rgb));
        }

        for i in 1..=segments {
            indices.push(0);
            indices.push(i);
            indices.push((i % segments) + 1);
        }

        let shape = renderer.create_shape(Shape { vertices, indices });
        renderer.add_instance(shape, [0.0, 0.0], [1.0, 1.0]);
    }

    pub fn create_text(
        &self,
        renderer: &mut WgpuRenderer,
        anchor: Point,
        content: impl Into<String> + Copy,
        font_family: Family<'static>,
        font_size: f32,
        line_height: f32,
        bounds: Option<(Measurement, Measurement)>,
        color: Color,
    ) {
        let [x, y] = anchor.to_screen_space(renderer.size);
        let content_str = content.into();

        let (text_width, text_height) = match bounds {
            Some((w_measure, h_measure)) => (
                w_measure.to_size(renderer.size.width),
                h_measure.to_size(renderer.size.height),
            ),
            None => renderer.measure_text_size(
                &content_str,
                font_family,
                font_size,
                line_height,
            )
        };

        let left = x - text_width / 2.0;
        let top = y - text_height / 2.0;

        let text_instance = TextInstance {
            content: content_str,
            font_family,
            font_size,
            line_height,
            text_area: TextInstanceArea {
                left,
                top,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: renderer.size.width as i32,
                    bottom: renderer.size.height as i32,
                },
                color: color.into()
            },
        };

        renderer.add_text(text_instance);
    }

    pub fn create_rounded_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        center: Point,
        width: Measurement,
        height: Measurement,
        border_radius: Measurement,
        color: Color,
        segments_per_corner: u16,
    ) {
        let [center_x, center_y] = center.to_ndc(renderer.size);
        let width_pixels = width.to_size(renderer.size.width);
        let height_pixels = height.to_size(renderer.size.height);
        let width_ndc = width_pixels / renderer.size.width as f32 * 2.0;
        let height_ndc = height_pixels / renderer.size.height as f32 * 2.0;
        let half_width = width_ndc / 2.0;
        let half_height = height_ndc / 2.0;

        let radius_pixels = border_radius.to_size(renderer.size.width);
        let radius_ndc_x = radius_pixels / renderer.size.width as f32 * 2.0;
        let radius_ndc_y = radius_pixels / renderer.size.height as f32 * 2.0;

        let radius_ndc_x = radius_ndc_x.min(half_width);
        let radius_ndc_y = radius_ndc_y.min(half_height);

        let rgb = color.into();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let top_left_center = [center_x - half_width + radius_ndc_x, center_y + half_height - radius_ndc_y];
        let top_right_center = [center_x + half_width - radius_ndc_x, center_y + half_height - radius_ndc_y];
        let bottom_left_center = [center_x - half_width + radius_ndc_x, center_y - half_height + radius_ndc_y];
        let bottom_right_center = [center_x + half_width - radius_ndc_x, center_y - half_height + radius_ndc_y];

        vertices.push(Vertex::new([center_x, center_y], rgb));
        let center_idx = 0;

        let left_mid = [center_x - half_width, center_y];
        let right_mid = [center_x + half_width, center_y];
        let top_mid = [center_x, center_y + half_height];
        let bottom_mid = [center_x, center_y - half_height];

        let top_left_h = [center_x - half_width, center_y + half_height - radius_ndc_y];
        let top_left_v = [center_x - half_width + radius_ndc_x, center_y + half_height];

        let top_right_h = [center_x + half_width, center_y + half_height - radius_ndc_y];
        let top_right_v = [center_x + half_width - radius_ndc_x, center_y + half_height];

        let bottom_left_h = [center_x - half_width, center_y - half_height + radius_ndc_y];
        let bottom_left_v = [center_x - half_width + radius_ndc_x, center_y - half_height];

        let bottom_right_h = [center_x + half_width, center_y - half_height + radius_ndc_y];
        let bottom_right_v = [center_x + half_width - radius_ndc_x, center_y - half_height];

        let mut vertex_indices = Vec::new();

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(top_left_v, rgb));

        for i in 0..=segments_per_corner {
            let angle = std::f32::consts::PI + (i as f32 / segments_per_corner as f32) * std::f32::consts::FRAC_PI_2;
            let x = top_left_center[0] + radius_ndc_x * angle.cos();
            let y = top_left_center[1] + radius_ndc_y * angle.sin();
            vertex_indices.push(vertices.len() as u16);
            vertices.push(Vertex::new([x, y], rgb));
        }

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(top_left_h, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(left_mid, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(bottom_left_h, rgb));

        for i in 0..=segments_per_corner {
            let angle = std::f32::consts::FRAC_PI_2 + (i as f32 / segments_per_corner as f32) * std::f32::consts::FRAC_PI_2;
            let x = bottom_left_center[0] + radius_ndc_x * angle.cos();
            let y = bottom_left_center[1] + radius_ndc_y * angle.sin();
            vertex_indices.push(vertices.len() as u16);
            vertices.push(Vertex::new([x, y], rgb));
        }

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(bottom_left_v, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(bottom_mid, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(bottom_right_v, rgb));

        for i in 0..=segments_per_corner {
            let angle = 0.0 + (i as f32 / segments_per_corner as f32) * std::f32::consts::FRAC_PI_2;
            let x = bottom_right_center[0] + radius_ndc_x * angle.cos();
            let y = bottom_right_center[1] + radius_ndc_y * angle.sin();
            vertex_indices.push(vertices.len() as u16);
            vertices.push(Vertex::new([x, y], rgb));
        }

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(bottom_right_h, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(right_mid, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(top_right_h, rgb));

        for i in 0..=segments_per_corner {
            let angle = std::f32::consts::FRAC_PI_2 * 3.0 + (i as f32 / segments_per_corner as f32) * std::f32::consts::FRAC_PI_2;
            let x = top_right_center[0] + radius_ndc_x * angle.cos();
            let y = top_right_center[1] + radius_ndc_y * angle.sin();
            vertex_indices.push(vertices.len() as u16);
            vertices.push(Vertex::new([x, y], rgb));
        }

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(top_right_v, rgb));

        vertex_indices.push(vertices.len() as u16);
        vertices.push(Vertex::new(top_mid, rgb));

        for i in 0..vertex_indices.len() - 1 {
            indices.push(center_idx);
            indices.push(vertex_indices[i]);
            indices.push(vertex_indices[i + 1]);
        }

        indices.push(center_idx);
        indices.push(vertex_indices[vertex_indices.len() - 1]);
        indices.push(vertex_indices[0]);

        let shape = renderer.create_shape(Shape { vertices, indices });
        renderer.add_instance(shape, [0.0, 0.0], [1.0, 1.0])
    }

    pub fn create_anchored_rounded_rectangle(
        &self,
        renderer: &mut WgpuRenderer,
        top_left: Point,
        width: Measurement,
        height: Measurement,
        border_radius: Measurement,
        color: Color,
        segments_per_corner: u16,
    ) {
        let [x, y] = top_left.to_screen_space(renderer.size);
        let width_px = width.to_size(renderer.size.width);
        let height_px = height.to_size(renderer.size.height);
        let center_x = x + width_px / 2.0;
        let center_y = y + height_px / 2.0;
        let center = Point::from_pixels(center_x, center_y);
        self.create_rounded_rectangle(renderer, center, width, height, border_radius, color, segments_per_corner)
    }
}
