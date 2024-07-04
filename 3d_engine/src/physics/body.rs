use cgmath::Point3;
use crate::{
	graphics::game::{
		vertex::Vertex,
		model::Mesh
	},
	physics::{
		collition::{
			CollitionBox,
            get_ray
		}
	}
};

// https://en.wikipedia.org/wiki/Physical_object
// It's useless and computionally expensive to have a physicalbody for each mesh
// This will just be used for each model

// GENERICAL ORDER TO USE THIS:
// start off by creating a Vec<Mesh>
// 	after loading the vertices and the indices, create the mesh's physics
//  SmallPhysicalBody::new_centroid(&vertices);
//  SmallPhysicalBody::get_mesh_area(&vertices, &indices);
// 	now create the Mesh!
// Then you'll have to create a Model:
//	you have a Vec<Mesh>, so just proceed by creating the model's physics
//  PhysicalBody::get_area(&meshes);
//  PhysicalBody::new_centroid(&area, &meshes);
// 	now create the Model!
// Compleated


// PHYSICS FOR MODELS 
pub struct PhysicalBody {
	pub mass: f32,
	pub speed: f32,
	pub area: f32,
	pub centroid: Point3<f32>,
    pub collition_box: CollitionBox
}

impl PhysicalBody {

    pub fn update(&mut self, meshes: &Vec<Mesh>) {
        self.centroid = Self::new_centroid(&self.area, meshes);
    }	

	pub fn new(mass: f32, speed: f32, meshes: &Vec<Mesh>) -> Self {
		
		let area = Self::get_area(meshes);
        let centroid = Self::new_centroid(&area, meshes);
	    let ray = get_ray(meshes, &centroid);	
        
        PhysicalBody {
			mass: mass,
			speed: speed,
			area: area.clone(),
			centroid,
            collition_box: CollitionBox::Sphere(ray)
		}
	}
	
	pub fn get_area(meshes: &Vec<Mesh>) -> f32 {
		
		let mut tot_area = 0.0;
		
		for mesh in meshes {
			tot_area += mesh.physics.area;
		}

        tot_area
	}
	
	pub fn new_centroid(area: &f32, meshes: &Vec<Mesh>) -> Point3<f32> {
		
		let (mut som_x, mut som_y, mut som_z) = (0.0, 0.0, 0.0);
		
		for mesh in meshes {
			let mesh_area = mesh.physics.get_importance(area);
			som_x += mesh.physics.centroid.x * mesh_area;
			som_y += mesh.physics.centroid.y * mesh_area;
			som_z += mesh.physics.centroid.z * mesh_area;
		}
		
		Point3::<f32> {
            x: som_x / 2.0,
            y: som_y / 2.0,
            z: som_z / 2.0
        }
	}
}

// PHYSICS FOR MESHES
#[derive(Debug)]
pub struct SmallPhysicalBody {
    pub	area: f32,
	pub centroid: Point3<f32>, 
}

impl SmallPhysicalBody {
	
	pub fn update(&mut self, vertices: &Vec<Vertex>) {
		self.centroid = Self::new_centroid(vertices)
	}
	
	pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
		SmallPhysicalBody {
			centroid: Self::new_centroid(vertices),
			area: Self::get_mesh_area(vertices, indices)
		}
	}
	
	fn get_importance(&self, tot_area: &f32) -> f32 {
		&self.area / tot_area
	}
	
	fn new_centroid(vertices: &Vec<Vertex> ) -> Point3<f32> {
		
		let (mut som_x, mut som_y, mut som_z) = (0.0, 0.0, 0.0);
		
		for vertex in vertices {
			som_x += vertex.position[0];
			som_y += vertex.position[1];
			som_z += vertex.position[2];
		}
		
		let len = vertices.len() as f32;
		
		Point3 {
			x: som_x / len,
			y: som_y / len,
			z: som_z / len
		}
	}
	
	fn get_mesh_area(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> f32 {
		
		let mut mesh_area = 0.0;
		println!("taglia dell indice: {:?}", indices.len());		
		println!("taglia dei vertici: {:?}", vertices.len());
		println!("dovrebbe essere 128 triangoli, {:?}", indices.chunks(3).len());
		for chunk in indices.chunks(3) {
			println!("{:?}", &chunk);
		}
		// for each triangle
		for chunk in indices.chunks(3) {
			//println!("{:?}", &chunk);			
			let (A, B, C) = (
				vertices[chunk.get(0).unwrap().clone() as usize],
				vertices[chunk.get(1).unwrap().clone() as usize],
				vertices[chunk.get(2).unwrap().clone() as usize]
			);
			
			// Erone's formula
			let a = ((A.position[0] - B.position[0]).powi(2) +
					(A.position[1] - B.position[1]).powi(2) +
					(A.position[2] - B.position[2]).powi(2)).sqrt();
					
			let b = ((B.position[0] - C.position[0]).powi(2) +
					(B.position[1] - C.position[1]).powi(2) +
					(B.position[2] - C.position[2]).powi(2)).sqrt();
			
			let c = ((C.position[0] - A.position[0]).powi(2) +
					(C.position[1] - A.position[1]).powi(2) +
					(C.position[2] - A.position[2]).powi(2)).sqrt();
		
			let p = (a + b + c) / 2.0;
			
			let triangle_area =  (p * (p - a) * (p - b) * (p - c)).sqrt();
			mesh_area += triangle_area;
		}

        mesh_area
	}
}
