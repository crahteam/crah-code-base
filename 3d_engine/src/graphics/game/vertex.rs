use crate::graphics::buffer;
use wgpu::{
    VertexBufferLayout,
    VertexStepMode,
    VertexFormat
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3]
}

impl <'a>Vertex {
    pub fn desc() -> VertexBufferLayout<'a> {
		
        buffer::create_vertex_buffer_layout(
            std::mem::size_of::<Vertex>(),
            &[
                buffer::vertex_attribute!(VertexFormat::Float32x3, 0, 0),
                buffer::vertex_attribute!(VertexFormat::Float32x2, std::mem::size_of::<[f32; 3]>(), 1),
                buffer::vertex_attribute!(VertexFormat::Float32x3, std::mem::size_of::<[f32; 5]>(), 2)
            ],
            VertexStepMode::Vertex
        )
        
    }
}
// This function returns a cleaned up vector with only unique vertices
// and a vector with the indices
pub fn unique_vertices_and_indices(dirty_vertices: Vec<Vertex>) -> (Vec<u32>, Vec<Vertex>) {
    println!("{:?}", dirty_vertices);
    let mut indices = vec![];
    let mut vertices = vec![];

    for vertex in dirty_vertices {

        match vertices.iter().position(|&i| i == vertex) {
            Some(i) => {
                // if its in the vec, then we only have to push the i
                indices.push(i as u32);
            },
            None =>{
                vertices.push(vertex);
                indices.push(vertices.len() as u32 - 1);
            }
        }        
    }

    println!("{} {}", indices.len() as u32, vertices.len() as u32);
    if (indices.len() as u32 % 3) != 0 {
        panic!("in unique vertices and indices the indices werent subdividable by 3");
    }
    (indices, vertices)
}
