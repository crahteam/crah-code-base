use cgmath::Point3;
use crate::physics::body::{
    PhysicalBody,
    SmallPhysicalBody,
};
use crate::graphics::{
    game::{
        vertex::Vertex
    },
    texture::TextureData
};
use wgpu::{
    Buffer
};


pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    // ! the model's collitionbox is different and has a different centroid from
    //   its meshes.
    // ! you might prefer to detect collitions by iterating over each mesh's collition
    //   boxes. Using the model's collition box performs better, b
    // pub collition_box: CollitionBox<'a>,
    pub physics: PhysicalBody
}

pub struct ModelDescriptor<'a> {
    pub dir: &'a str,
    pub name: &'a str,
    pub speed: f32,
    pub mass: f32,
    pub position: Point3::<f32> // this position is just the spawn point
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_elements: u32,
    pub material: usize,
    pub vertices: Vec<Vertex>,
    pub physics: SmallPhysicalBody
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub texture_data: TextureData
}
