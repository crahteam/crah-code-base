use winit::{
    event::VirtualKeyCode
};

use crate::core::{
	camera::CameraData,
	buffer,
	objmodel::{
		Model,
	},
	collition::{
		CollitionBox
	}
};

use wgpu::{
	BufferUsages,
	Device,
	Buffer
};

pub struct Controller {
    pub speed: f32,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            speed: 0.1,
            forward: false,
            back: false,
            left: false,
            right: false,
            up: false,
            down: false
        }
    }
}

impl Controller {
	
    pub fn update_controller(&mut self, keycode: Option<winit::event::VirtualKeyCode>) {
        self.forward = false;
        self.back = false;
        self.right = false;
        self.left = false;
        self.up = false;
        self.down = false;
        
        match keycode {
            Some(VirtualKeyCode::W) => {
               self.forward = true;
            },
            Some(VirtualKeyCode::A) => {
               self.left = true;
            },
            Some(VirtualKeyCode::D) => {
               self.right = true;
            },
            Some(VirtualKeyCode::S) => {
               self.back = true;
            },
			Some(VirtualKeyCode::Space) => {
				self.up = true;
			},
			Some(VirtualKeyCode::Z) => {
				self.down = true;
			}
            _ => {}
        }
    }


    pub fn update_on_movement<'a>(&self, device: &Device, camera_data: &mut CameraData, player: &mut Model) {
		
        if &self.forward == &true {
           camera_data.camera.eye.z -= &self.speed;
           camera_data.camera.target.z -= &self.speed;
            
            for mut mesh in &mut player.meshes {
				for mut vertex in &mut mesh.raw_vertices {
					vertex.position[2] -= &self.speed;
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
        };
        
        if &self.back == &true {
			
            camera_data.camera.eye.z += &self.speed;
            camera_data.camera.target.z += &self.speed;
               
            for mut mesh in &mut player.meshes {
				for mut vertex in  &mut mesh.raw_vertices {
					vertex.position[2] += &self.speed;
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
			
        };
        
        if &self.right == &true {
			  camera_data.camera.eye.x += &self.speed;
			  camera_data.camera.target.x += &self.speed;
			  
              for mut mesh in &mut player.meshes {
				for mut vertex in  &mut mesh.raw_vertices {
					vertex.position[0] += &self.speed
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
			
        };
        
        if &self.left == &true {
			 camera_data.camera.eye.x -= &self.speed;
			 camera_data.camera.target.x -= &self.speed;
			 
             for mut mesh in &mut player.meshes {
				for mut vertex in  &mut mesh.raw_vertices {
					vertex.position[0] -= &self.speed;
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
			
        };
        
        if &self.up == &true {
			 camera_data.camera.eye.y += &self.speed;
			 //camera_data.camera.target.x -= &self.speed;
			 
             for mut mesh in &mut player.meshes {
				for mut vertex in  &mut mesh.raw_vertices {
					vertex.position[1] += &self.speed;
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
        }
        
        if &self.down == &true {
			 camera_data.camera.eye.y -= &self.speed;
			 //camera_data.camera.target.x -= &self.speed;
			 
             for mut mesh in &mut player.meshes {
				for mut vertex in  &mut mesh.raw_vertices {
					vertex.position[1] -= &self.speed;
				}
				CollitionBox::update_from_vertices(&mut mesh);
			}
			
			
        }
		println!("{:?}", &player.meshes[0].collition_box);
		println!("{:?}", &player.meshes[0].collition_box.centroid);
		
		println!("{:?}", &camera_data.camera.target);
		
        //camera_data.camera.target.x = camera_data.camera.eye.x;
        //camera_data.camera.target.y = camera_data.camera.eye.y;
        //camera_data.camera.target.z = camera_data.camera.eye.z - 5.0;

		for mesh in &mut player.meshes {
			mesh.vertex_buffer = buffer::create_buffer(
				device,
				mesh.raw_vertices.clone(),
				BufferUsages::VERTEX
			);
		}
    }
}

// every time the camera moves, aka: the camera.eye position changes,
// we have to change the position of all the vertices of the player model and update its buffer
// with the new vertices.

#[macro_export]
macro_rules! update_player_buffer{
	($a: expr) => {
		{
			
		}
	}
}
