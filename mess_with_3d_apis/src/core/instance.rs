use cgmath::{
    Quaternion,
    Vector3,
    Matrix4
};
use wgpu::{
    Buffer,
    VertexBufferLayout,
    VertexFormat
};
use crate::core::{
    buffer,
    buffer::{
        vertex_attr
    }
};
pub struct InstanceData {
    pub instances: Vec<Instance>,
    pub instance_buffer: Buffer
}

pub struct Instance {
    pub position: Vector3<f32> ,
    pub rotation: Quaternion<f32>
}


impl Instance {
    // from Instance to InstanceRaw conversion.
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)).into()
        }
    }
}

// what the shader will receive is just a Matrix4
// as an array of 4 arrays with 4 values each
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4]  
}

impl <'a>InstanceRaw {
    pub fn desc() -> VertexBufferLayout<'a> {
        let ivbl = buffer::create_vertex_buffer_layout(
            std::mem::size_of::<InstanceRaw>(),
            &[
                vertex_attr!(VertexFormat::Float32x4, 0, 5),
                vertex_attr!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 4]>(), 6),
                vertex_attr!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 8]>(), 7),
                vertex_attr!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 12]>(), 8)
            ],
            wgpu::VertexStepMode::Instance
        );
        ivbl
    }
}
