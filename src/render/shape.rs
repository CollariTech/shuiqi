use glyphon::TextBounds;
use wgpu::Buffer;
use crate::drawer::color::Color;
use crate::shaders::{InstanceData, Vertex};

#[derive(Clone, Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapeData {
    pub shape_id: u32,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_count: u32
}

pub struct ObjectInstance {
    pub shape: ShapeData,
    pub data: InstanceData
}

#[derive(Clone, Debug)]
pub struct TextInstance {
    pub content: String,
    pub font_family: glyphon::Family<'static>,
    pub font_size: f32,
    pub line_height: f32,
    pub text_area: TextInstanceArea
}

#[derive(Clone, Debug)]
pub struct TextInstanceArea {
    pub left: f32,
    pub top: f32,
    pub scale: f32,
    pub bounds: TextBounds,
    pub color: Color
}

impl ObjectInstance {
    pub fn new(shape: ShapeData, data: InstanceData) -> Self {
        ObjectInstance { shape, data }
    }
}

