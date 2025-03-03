use crate::painter::color::Color;
use crate::painter::point::{Measurement, Point};
use crate::painter::Object;
use crate::render::shape::{Shape, TextInstance, TextInstanceArea};
use crate::render::wgpu::WgpuRenderer;
use crate::shaders::Vertex;
use glyphon::{Family, TextBounds};

pub fn draw_objects(renderer: &mut WgpuRenderer, objects: Vec<Object>) -> Vec<u32> {
    let mut ids = Vec::new();
    for object in objects {
        let object_shapes = draw_object(renderer, object);
        ids.extend(object_shapes);
    }
    ids
}

pub fn draw_object(renderer: &mut WgpuRenderer, object: Object) -> Vec<u32> {
    let mut ids = Vec::new();
    if object.fill_color.is_none() {
        return ids;
    }
    let [x, y] = object.screen_point.to_screen_space(renderer.size);
    let width_px = object.width.transform_with_bound(renderer.size.width);
    let height_px = object.height.transform_with_bound(renderer.size.height);
    let screen_point = Point::positioned(
        x,
        y,
        width_px,
        height_px,
        object.reference_corner,
    );

    let fill_color = object.fill_color.unwrap();
    if let Some(border_radius) = object.border_radius {
        ids.push(create_rounded_rectangle(
            renderer,
            screen_point,
            object.width,
            object.height,
            border_radius,
            fill_color,
            64 // placeholder value
        ));
    } else {
        ids.push(create_rectangle(
            renderer,
            screen_point,
            object.width,
            object.height,
            fill_color
        ));
    }

    if let Some(inner_text) = object.text {
        let text_size = renderer.measure_text_size(
            &inner_text.content,
            inner_text.font_family,
            inner_text.font_size,
            inner_text.line_height,
        );
        let text_position = Point::positioned(
            x,
            y + text_size.1 / 4.0,
            width_px,
            height_px,
            inner_text.corner.inverted()
        );
        println!("Inner corner: {:?}", inner_text.corner);

        create_text(
            renderer,
            text_position,
            &inner_text.content,
            inner_text.font_family,
            inner_text.font_size,
            inner_text.line_height,
            Some(text_size),
            inner_text.color
        );
    }
    ids
}


pub fn create_rectangle(
    renderer: &mut WgpuRenderer,
    center: Point,
    width: Measurement,
    height: Measurement,
    color: Color
) -> u32 {
    let [center_x, center_y] = center.to_ndc(renderer.size);
    let width_pixels = width.transform_with_bound(renderer.size.width);
    let height_pixels = height.transform_with_bound(renderer.size.height);
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


pub fn create_circle(
    renderer: &mut WgpuRenderer,
    center: Point,
    radius: Measurement,
    color: Color,
    segments: u16,
) -> u32 {
    let [center_x, center_y] = center.to_ndc(renderer.size);

    let radius_pixels = radius.transform_with_bound(renderer.size.width);

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
    renderer.add_instance(shape, [0.0, 0.0], [1.0, 1.0])
}

pub fn create_rounded_rectangle(
    renderer: &mut WgpuRenderer,
    center: Point,
    width: Measurement,
    height: Measurement,
    border_radius: Measurement,
    color: Color,
    segments_per_corner: u16
) -> u32 {
    let [center_x, center_y] = center.to_ndc(renderer.size);
    let width_pixels = width.transform_with_bound(renderer.size.width);
    let height_pixels = height.transform_with_bound(renderer.size.height);
    let radius_pixels = border_radius.broken_transform_with_bound(width_pixels);

    let is_circle = (width_pixels == height_pixels) && (width_pixels == 2.0 * radius_pixels);
    if is_circle {
        return create_circle(renderer, center, Measurement::Pixels(radius_pixels), color, segments_per_corner);
    }

    let width_ndc = width_pixels / renderer.size.width as f32 * 2.0;
    let height_ndc = height_pixels / renderer.size.height as f32 * 2.0;
    let half_width = width_ndc / 2.0;
    let half_height = height_ndc / 2.0;

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

pub fn create_text(
    renderer: &mut WgpuRenderer,
    anchor: Point,
    content: impl Into<String> + Copy,
    font_family: Family<'static>,
    font_size: f32,
    line_height: f32,
    bounds: Option<(f32, f32)>,
    color: Color,
) {
    let [x, y] = anchor.to_screen_space(renderer.size);
    let content_str = content.into();

    let (text_width, text_height) = bounds.unwrap_or_else(|| {
        renderer.measure_text_size(
            &content_str,
            font_family,
            font_size,
            line_height,
        )
    });

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