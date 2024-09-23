use bytemuck::{Pod, Zeroable};

pub mod pipeline;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3]
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}


const COLOR: [f32; 3] = [0.5, 0.5, 0.5];

pub fn square() -> (&'static [Vertex], &'static [u16]) {
    let vertices: &[Vertex] = &[
        Vertex { position: [-0.8, 0.8], color: COLOR },
        Vertex { position: [0.8, 0.8], color: COLOR },
        Vertex { position: [0.8, -0.8], color: COLOR },
        Vertex { position: [-0.8, -0.8], color: COLOR }
    ];

    let indices: &[u16] = &[
        1, 0, 3,
        3, 2, 1
    ];

    unsafe {
        (std::mem::transmute(vertices), std::mem::transmute(indices))
    }
}