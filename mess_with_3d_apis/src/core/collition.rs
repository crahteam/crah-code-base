use crate::core::{
	vertex::{
		CoordsRange,
		CoordsBool,
		ModelVertex
	},
	objmodel::{
		Mesh
	}
};
use std::cmp::{
	min,
	max
};

use std::ops::Range;

use cgmath::{
	Point3
};

#[derive(Debug)]
pub struct CollitionBox<'a> {
	pub name: &'a str,
	pub coords: CoordsRange,
	pub centroid: Point3::<f32>,
	pub last_position: CoordsBool,
}

impl CollitionBox<'_> {
	
	pub fn detect_collition(&mut self, cbox: &CollitionBox) -> bool {
		if CoordsRange::overlap(&self.coords.x, &cbox.coords.x) &&
		   CoordsRange::overlap(&self.coords.y, &cbox.coords.y) &&
		   CoordsRange::overlap(&self.coords.z, &cbox.coords.z) {
			//self.react(cbox);
			true
		} else {
			false
		}
	}
	
	pub fn react(&mut self, cbox: &CollitionBox) {
		println!("REACTION FROM COLLISION");
		
	}
	
	pub fn new_from_meshes<'a>(name: &'a str, meshes: &Vec<Mesh<'a>>) -> Self {
		let mut fake_box = Self {
			name: "sronzo ancora piu grosso",
			coords: CoordsRange {
				x: (0.)..0., y: (0.)..0., z: (0.)..0.,
			},
			centroid: Point3::<f32>{x: 0.0, y: 0.0, z: 0.0},
			last_position: CoordsBool {
				x: false,
				y: false,
				z: false
			}
		};
		
		fake_box.update_from_meshes("ima baby girl", meshes);
		
		fake_box
	}
	
	pub fn update_from_meshes<'a>(&mut self,name: &str, meshes: &Vec<Mesh<'a>>) {
		self.update_coords_from_meshes(name, meshes);
		self.update_centroid();
	}
	
	pub fn update_coords_from_meshes<'a>(&mut self,name: &'a str, meshes: &Vec<Mesh<'a>>) {
		
		let (mut x, mut y, mut z): (Range::<f32>, Range::<f32>, Range::<f32>)  = ((0.)..(0.), (0.)..(0.), (0.)..(0.));
		
		for mesh in meshes {
			
			x.start = x.start.min(mesh.collition_box.coords.x.start);
			x.end = x.end.max(mesh.collition_box.coords.x.end);
			
			z.start = z.start.min(mesh.collition_box.coords.z.start);
			z.end = z.end.max(mesh.collition_box.coords.z.end);
			
			y.start = y.start.min(mesh.collition_box.coords.y.start);
			y.end = y.end.max(mesh.collition_box.coords.y.end);
			
		}
		
		self.coords = CoordsRange {x, y, z};
		
	}
	
	pub fn new_from_vertices<'a>(name: &'a str) -> Self {
		
		// we create a fake Collition Box and update its values
		
		let mut fake_box = Self {
			name: "stronzo",
			coords: CoordsRange {
				x: (0.)..0., y: (0.)..0., z: (0.)..0.,
			},
			centroid: Point3::<f32>{x: 0.0, y: 0.0, z: 0.0},
			last_position: CoordsBool {
				x: false, y: false, z: false
			}
		};
		
		// now the fake box has real values
		
		fake_box
	}
	
	// run this to update the whole CollitionBox quickly
	pub fn update_from_vertices(mesh: &mut Mesh){
		mesh.collition_box.update_coords_from_vertices(&mesh.raw_vertices);
		mesh.collition_box.update_centroid();
	}
	
	pub fn update_coords_from_vertices(&mut self, vertices: &Vec<ModelVertex>) {
		
		let (mut x, mut y, mut z): (Range::<f32>, Range::<f32>, Range::<f32>) = ((0.)..(0.), (0.)..(0.), (0.)..(0.));
		
		for vertex in vertices {
		
			x.end = x.end.max(vertex.position[0]);
			x.start = x.start.min(vertex.position[0]);
			
			y.end = y.end.max(vertex.position[1]);
			y.start = y.start.min(vertex.position[1]);
			
			z.end = z.end.max(vertex.position[2]);
			z.start = z.start.min(vertex.position[2]);
			
		}
		
		self.coords = CoordsRange {x, y, z};
	}
	
	// ! make sure to use this only after having updated the coords
	pub fn update_centroid(&mut self) {
		
		self.centroid.x = (&self.coords.x.start + &self.coords.x.end) / 2.0;
		self.centroid.y = (&self.coords.y.start + &self.coords.y.end) / 2.0; 
		self.centroid.z = (&self.coords.z.start + &self.coords.z.end) / 2.0;
			
	}
	
}

