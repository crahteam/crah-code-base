use crate::graphics::{
    game::model::{
        Model,
        Mesh
    },
    game::vertex::Vertex,
    buffer
};
use wgpu::{
    BufferUsages,
    Device
};

use cgmath::{
    Vector3,
    Deg,
    Rad,
    Point3,
    InnerSpace,
    prelude::Angle
};
pub struct Movement {
	vec: Vector3::<f32>, // this defines the delta of the movement
    //rotation: Quaternion
}

pub struct Rotation {
    axis: Vector3::<f32>, // get this by cgmath::Vector3::unit_y();
    angle: Deg::<f32>
}

#[derive(PartialEq)]
pub enum BasicDirection {
    Forward,
    Back,
    Left,
    Right
}

impl Movement {
    // THIS IS created for a mesh, considering 
    // NOTE: to move an entity theres an implementation on the entity itself
    
    pub fn move_model(&self, device: &Device, model: &mut Model) {
        
        for mut mesh in &mut model.meshes {
            Self::move_mesh(&self, device, &mut mesh);
        }

        model.physics.update(&model.meshes);
    }

    pub fn move_mesh(&self, device: &Device, mesh: &mut Mesh) {

        for mut vertex in &mut mesh.vertices {
            Self::move_vertex(&self, &mut vertex);
        } 

        mesh.vertex_buffer = buffer::create_buffer(
            device,
            mesh.vertices.clone(),
            BufferUsages::VERTEX
        );

        mesh.physics.update(&mesh.vertices);
    }

    pub fn move_vertex(&self, vertex: &mut Vertex) {
       vertex.position[0] += &self.vec.x;
       vertex.position[1] += &self.vec.y;
       vertex.position[2] += &self.vec.z;
       // NOTE: we still have to find out what to put around the axis
       //vertex.position = Quaternion::from_axis_angle(&vertex.position.normalize(), &self.rotation.angle).into();
    }

    pub fn move_point(&self, point: &mut cgmath::Point3::<f32>) {
       point.x += &self.vec.x;
       point.y += &self.vec.y;
       point.z += &self.vec.z;
       // NOTE: we still have to find out what to put around the axis
       //vertex.position = Quaternion::from_axis_angle(&vertex.position.normalize(), &self.rotation.angle).into();
    }

    // this function retrives a movement vector given a yaw angle
    pub fn yaw_movement(angle: Rad::<f32>, dir: BasicDirection, amount: f32) -> Self {
        let (a_sin, a_cos) = angle.sin_cos();
        let mut vector: Vector3::<f32>;
        match dir {
            // NOTE: if in the passed code values are negative there might be no need to have Left
            // and back considering they would get the opposite of right and forward already?
            BasicDirection::Forward => {
                vector = Vector3::new(a_cos, 0.0, a_sin).normalize();
            },
            BasicDirection::Right => {
                vector = Vector3::new(-a_sin, 0.0, a_cos).normalize();
            },
            BasicDirection::Left => {
                vector = Vector3::new(a_sin, 0.0, -a_cos).normalize();
            },
            BasicDirection::Back => {
                vector = Vector3::new(-a_cos, 0.0, -a_sin).normalize();
            }
        }
        // now the vector has the right direction in 2d plan
        vector *= amount;
        Self {
            vec: vector
        }
    }
    
    // USEFUL TO MOVE AN ENTITY TO A POINT USING ITS CENTROID
    pub fn movement_from_points(a: &Point3::<f32>, b: &Point3::<f32>) -> Self {
        let vector = cgmath::Vector3::<f32>::new(a.x - b.x, a.y - b.y, a.z - b.z);
        Self {
            vec: vector
        }
    }
}
// to create let rotation = cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0));

