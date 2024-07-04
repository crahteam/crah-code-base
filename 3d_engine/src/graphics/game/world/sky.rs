use wgpu::{
	RenderPipeline,
};

use crate::graphics::{
	instance::{
		InstanceData
	},
};
pub enum SkyBackground {
    Rgba([f64; 3])
}

pub struct Sky {
    pub background: SkyBackground,
    pub ray: f32 //this ray corresponds like to the atmosphere barrier
}


