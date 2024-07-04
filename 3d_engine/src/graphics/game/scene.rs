use std::collections::HashMap;
use crate::graphics::{
	game::{
		entity::{
			Entity,
		},
		world::World,
        controller::Controller,
        camera::CameraData
	},
    texture::TextureData
};

use wgpu::{
    RenderPipeline,
    BindGroup
};
use crate::audio::Audio;
use crate::user::{
    Role,
    Pipe,
    BGS
};

pub struct Scene {
	pub entities: HashMap<Role, Entity>,
    pub pipelines: HashMap<Pipe, RenderPipeline>,
    pub bindgroups: HashMap<BGS, BindGroup>,
   	pub controller: Controller,
	pub cameras: HashMap<String, CameraData>,
	pub world: World, 
    pub audio: Audio,
    
}

// to a ROLE theres a corresponding PIPE
// Entity Pipe Role
// the entity struct, which pipeline to use of the created ones, whats the role of the entity
//
// role -> pipe is inside roles for each role (Pipe)
