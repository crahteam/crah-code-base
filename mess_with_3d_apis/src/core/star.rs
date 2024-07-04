use wgpu::{
	RenderPipeline,
};
use crate::core::{
	instance::{
		InstanceData
	},
	objmodel::{
		Model
	}
};

pub struct Stars<'a> {
	pub model: Model<'a>,
	pub instance_data: InstanceData,
	pub pipeline: RenderPipeline,
	pub ray: f32
}

