use bytemuck::{Pod, Zeroable};
use wgpu::Buffer;
use crate::graphics::Vertex;

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
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
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

pub struct ShapeData {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_count: u32
}

pub struct ObjectInstance {
    pub shape: ShapeData,
    pub data: InstanceData
}

impl ObjectInstance {
    pub fn new(shape: ShapeData, data: InstanceData) -> Self {
        ObjectInstance { shape, data }
    }
}

