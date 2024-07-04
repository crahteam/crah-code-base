use crate::core::{
	objmodel::{
		Mesh,
	},
	vertex::{
		CoordsRange
	}
};
use winit::{
    event::VirtualKeyCode
};
use std::ops::Range;

pub struct Interaction<'a> {
	pub mesh: &'a str,
	pub target_mesh: &'a str,
	pub key: VirtualKeyCode,
	pub action: Action,
	pub ray: f32,
	pub state: PressState
}

#[derive(PartialEq)]
pub enum PressState {
	Pressed,
	Continuous,
	Released
}

impl Interaction<'_> {
	pub fn drop(self) {
	}
	pub fn should_we_interact(&self, mesh: &mut Mesh, target_mesh: &mut Mesh) -> bool {
		
		let x = Range::<f32> {
			start: target_mesh.collition_box.centroid.x - &self.ray,
			end: target_mesh.collition_box.centroid.x + &self.ray
		};
		
		let y = Range::<f32> {
			start: target_mesh.collition_box.centroid.y - &self.ray,
			end: target_mesh.collition_box.centroid.y + &self.ray
		};
		
		let z = Range::<f32> {
			start: target_mesh.collition_box.centroid.z - &self.ray,
			end: target_mesh.collition_box.centroid.z + &self.ray
		};
		
		if x.contains(&mesh.collition_box.centroid.x) &&
		   y.contains(&mesh.collition_box.centroid.y) &&
		   z.contains(&mesh.collition_box.centroid.z) {
				   
				return true;
		} else {
			return false;
		}
	}
}

pub enum Action {
	Print,
	RotateMonkey
}

impl Action {
	
}
