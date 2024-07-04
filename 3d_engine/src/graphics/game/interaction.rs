// If the t
use crate::user::{
    Role,
    Actions
};
use crate::graphics::game::{
    entity::Entity
};
use anyhow::{
    Error,
    bail
};
use crate::physics::collition::{
    spheres_detection
};
use crate::errors::game::InteractionError;
use std::collections::HashMap;
pub enum InteractionState {
    Contiuous, // the interaction has already started 
    Absent  // the interaction hasn't started yet. if this is the state
            // then make a sound and switch to continuous
}

pub struct Interaction {
    pub action: Actions,
    pub ray: f32,
    pub initiator: Role,
    pub target: Role,
}

impl Interaction {

    pub fn detect_interaction(&self, hashmap: &HashMap<Role, Entity>) -> Result<bool, Error> {
        let initiator_physics = match hashmap.get(&self.initiator) {
            Some(entity) => {
                &entity.model.physics
            },
            None => bail!(InteractionError::InitiatorNotFound)
        };
        let target_physics = match hashmap.get(&self.target) {
            Some(entity) => {
                &entity.model.physics
            },
            None => bail!(InteractionError::TargetNotFound)
        };

        Ok(spheres_detection(initiator_physics, target_physics))
    }
}

pub struct Action {
    pub state: InteractionState
}


