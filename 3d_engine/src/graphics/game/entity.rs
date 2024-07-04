use crate::graphics::game::{
    model::{
        Model,
        ModelDescriptor
    }
};
use cgmath::{
    Point3
};
use crate::utils::obj_loader;
use wgpu::{
    Device,
    Queue,
    BindGroupLayout
};
use crate::user::Pipe;
use crate::graphics::instance::{
    InstanceData,
    InstanceDescriptor
};
pub struct Entity {
	pub model: Model,
    pub instance_data: Option<InstanceData>,
    pub pipeline: Pipe // which pipeline to use
}
use anyhow::{
    Error,
    bail
};

use crate::physics::{
    movement::Movement
};

impl Entity {

    pub fn spawn(device: &Device, queue: &Queue, texture_bgl: &BindGroupLayout, desc: ModelDescriptor, pipe: Pipe) -> Result<Self, Error>{
        let model = pollster::block_on(obj_loader::load(device, queue, texture_bgl, &desc))?;
        let mut entity = Self::new(model, None, pipe);
        entity.move_to_point(device, &desc.position);
        Ok(entity)
    }

    pub fn spawn_instanced(device: &Device, queue: &Queue, texture_bgl: &BindGroupLayout, desc: ModelDescriptor, inst_desc: InstanceDescriptor, pipe: Pipe) -> Result<Self, Error> {
        let model = pollster::block_on(obj_loader::load(device, queue, texture_bgl, &desc))?;
        let instance_data = InstanceData::new(device, inst_desc);
        Ok(Self::new(model, Some(instance_data), pipe))
    }

    pub fn new(model: Model, instance_data: Option<InstanceData>, pipeline: Pipe) -> Self{
        Entity {
            model,
            instance_data,
            pipeline
        }
    }

	pub fn move_around(&mut self, device: &Device, movement: &Movement) {
	    &movement.move_model(device, &mut self.model);
	}

    pub fn move_to_point(&mut self, device: &Device, point: &Point3::<f32>) {
        let movement = Movement::movement_from_points(&self.model.physics.centroid, point);
        &mut self.move_around(device, &movement);
    }
}
