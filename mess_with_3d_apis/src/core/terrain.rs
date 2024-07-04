use crate::core::{
	image::{
		ImageDataLuma,
	},
	collition::{
		CollitionBox
	},
	objmodel::{
		Mesh,
		Model
	},
	vertex::{
		ModelVertex
	},
	buffer
};
use image::{
	GenericImageView
};

use wgpu::{
	Device,
	BufferUsages
};

pub struct Terrain<'a> {
	pub height_map: ImageDataLuma,
	pub plane: Model<'a>
}

impl<'a> Terrain<'a> {
	
	const MAX_HEIGHT: f32 = 10.0;
	const COLOR_SPACE: f32 = 255.0;
	
	pub fn new(device: &Device, mut plane: Model<'a>, height_map: ImageDataLuma) -> Self {
		// take in mind the plane should be one mesh only anyway
		for mut mesh in &mut plane.meshes {
			for mut vertex in <&mut Vec<ModelVertex> as Into<&mut Vec<ModelVertex>>>::into(&mut mesh.raw_vertices) { 
				
				// we make the vertex position relative to the image center 
				let x = vertex.position[0] as u32 + (height_map.size.0 / 2);
				let z = vertex.position[2] as u32 + (height_map.size.1 / 2);
				
				// we get the pixel with the same coords of the vertex (from the image center)
				let pixel = height_map.luma.get_pixel(x,z);
				
				// depending on the pixel's color from 0 to 255 sets the height
				vertex.position[1] = (pixel.0[0] as f32 / Self::COLOR_SPACE * Self::MAX_HEIGHT) as f32;
				
				// we make the mesh lower
				vertex.position[1] -= 10.5;
			}
			
			mesh.vertex_buffer = buffer::create_buffer(
				device,
				mesh.raw_vertices.clone(),
				BufferUsages::VERTEX
			);
				
			CollitionBox::update_from_vertices(&mut mesh);
		}
		
		Terrain {
			height_map, plane
		}
	}
}
