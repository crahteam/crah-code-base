use cgmath::{
    Quaternion,
    Vector3,
    Matrix4,
    prelude::Rotation3,
    InnerSpace,
    Point3
};

use rand::Rng;

use wgpu::{
    Buffer,
    VertexFormat,
    VertexBufferLayout,
    Device,
    BufferUsages
};

use crate::graphics::{
    buffer,
    buffer::vertex_attribute,
};

use std::ops::Range;

pub enum Displacement {
    Random(u32, Range::<f32>),
    //distance and per row
    Squared(u32, f32), // the distance between two instances
}

pub struct InstanceDescriptor {
    pub number: u32,
    pub position: Point3::<f32>,
    pub displacement: Displacement,
}

pub struct InstanceData {
    pub instances: Vec<Instance>,
    pub instance_buffer: Buffer,
}

impl InstanceData {
    pub fn new(device: &Device, desc: InstanceDescriptor) -> Self {
        //let position: cgmath::Vector3;
        let mut instances: Vec<Instance>;
        match desc.displacement {

            Displacement::Squared(rows, distance) => {
                instances = (0..rows).flat_map(|z| {
                    (0..rows).map(move |x| {
                       let x = distance * (x as f32 - desc.position.x / 2.0);
                       let z = distance * (z as f32 - desc.position.z / 2.0);
                       let position = cgmath::Vector3{x,y: 0.0, z};
                       let rotation = cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0));
                       Instance {
                            position,
                            rotation
                       }
                    })
                }).collect::<Vec<_>>()
            },

            Displacement::Random(tot, range) => {
                    instances = (0..tot).map(|z| {
                       let mut rng = rand::thread_rng();
                       let x = rng.gen_range(range.clone());
                       let z = rng.gen_range(range.clone());
                       let position = cgmath::Vector3{x,y: 0.0, z};
                       let rotation = cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0));
                       Instance {
                            position,
                            rotation
                       }
                    }).collect::<Vec<_>>()
            }
        }
        
        let raw_instances_vec = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        let instance_buffer = buffer::create_buffer(
            &device,
            raw_instances_vec,
            BufferUsages::VERTEX
        );

        Self {
            instances,
            instance_buffer
        }
    }
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
        buffer::create_vertex_buffer_layout(
            std::mem::size_of::<InstanceRaw>(),
            &[
                vertex_attribute!(VertexFormat::Float32x4, 0, 5),
                vertex_attribute!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 4]>(), 6),
                vertex_attribute!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 8]>(), 7),
                vertex_attribute!(VertexFormat::Float32x4, std::mem::size_of::<[f32; 12]>(), 8)
            ],
            wgpu::VertexStepMode::Instance
        )
    }
}
