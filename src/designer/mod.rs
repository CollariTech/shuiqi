pub mod point;

use glyphon::{Family, TextBounds};
use crate::designer::point::{Measurement, Point};
use crate::graphics::instance::{Shape, TextInstance, TextInstanceArea};
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

    pub fn create_text(
        &self,
        renderer: &mut WgpuRenderer,
        anchor: Point,
        content: impl Into<String> + Copy,
        font_family: Family<'static>,
        font_size: f32,
        line_height: f32,
        bounds: Option<(Measurement, Measurement)>,
        color: [u8; 4],
    ) {
        let [x, y] = anchor.to_screen_space(renderer.size);
        let content_str = content.into();

        let (text_width, text_height) = match bounds {
            Some((w_measure, h_measure)) => (
                w_measure.to_size(renderer.size.width),
                h_measure.to_size(renderer.size.height),
            ),
            None => {
                let mut temp_buffer = glyphon::Buffer::new(
                    &mut renderer.font_system,
                    glyphon::Metrics::new(font_size, line_height),
                );
                temp_buffer.set_size(&mut renderer.font_system, None, None);
                temp_buffer.set_text(
                    &mut renderer.font_system,
                    &content_str,
                    glyphon::Attrs::new().family(font_family),
                    glyphon::Shaping::Advanced,
                );
                let runs: Vec<_> = temp_buffer.layout_runs().collect();
                if runs.is_empty() {
                    (0.0, 0.0)
                } else {
                    let max_width = runs.iter().map(|run| run.line_w).fold(0.0, f32::max);
                    let last_run = runs.last().unwrap();
                    let total_height = last_run.line_y + last_run.line_height;
                    (max_width, total_height)
                }
            }
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
                color,
            },
        };

        renderer.add_text(text_instance);
    }
}
