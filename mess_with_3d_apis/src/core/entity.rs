use crate::core::{
	camera::{
		CameraStyle,
		CameraData
	}
};

use cgmath::{
	Point3
};

pub struct Entity {
	name: String,
	model: Model,
	camera: CameraData,
	mass: u32,
	speed: u32
}

impl Entity {
	
	pub fn spawn(desc: EntityDescriptor) -> Self {
		let name = "fuk entity".to_string();
		let model = load_obj_model(device, queue, tbgl, &name[..]).unwrap();
		let camera = CameraData,
		
		Entity {
			name, model, camera, mass: 23, speed: 3
		}
	}
	
	pub fn move(&mut self, device: &Device, position: Point3) {
		
		for mut mesh in &mut self.model.meshes {
			
			for mut vertex in &mut mesh.raw_vertices {
				// we make each vertex to be relative to the model centroid
				vertex.position -= model.meshes[0].collition_box.centroid;
				// we update the vertices adding the new position
				vertex.position += position;
			}
			
			// we still have to update the vertex_buffer with the new vertices
			
			mesh.vertex_buffer = buffer::create_buffer(
				device,
				&mesh.raw_vertices.clone(),
				BufferUsages::VERTEX
			);
		}
		
	}
}

pub struct EntityDescriptor<'a> {
	dir: &'a str,
	model_file: &'a str,
	camera_style: CameraStyle,
	position: Point3
}
	
