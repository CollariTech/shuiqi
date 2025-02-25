use crate::graphics::Vertex;
use bytemuck::{Pod, Zeroable};
use glyphon::TextBounds;
use std::mem::size_of;
use wgpu::Buffer;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct InstanceData {
    pub position: [f32; 2],
    pub scale: [f32; 2]
}

impl InstanceData {
    pub fn new(position: [f32; 2], scale: [f32; 2]) -> Self {
        InstanceData { position, scale }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

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
    pub color: [u8; 4]
}

impl ObjectInstance {
    pub fn new(shape: ShapeData, data: InstanceData) -> Self {
        ObjectInstance { shape, data }
    }
}

