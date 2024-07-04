use std::ops::Range;
pub const F32_MIN: f32 = std::f32::MIN;
pub const F32_MAX: f32 = std::f32::MAX;
use cgmath::{
    Vector3,
    Point3
};
use crate::graphics::game::{
    entity::Entity,
    model::{
        Mesh,
        Model
    }
};
use crate::user::Role;
use crate::physics::{
    body::{
        PhysicalBody,
        SmallPhysicalBody
    }
};
use std::collections::HashMap;
#[derive(PartialEq)]
pub struct CoordsRange {
	x: Range::<f32>,
	y: Range::<f32>,
	z: Range::<f32>
}

#[derive(PartialEq)]
pub enum CollitionBox {
	// the ray is the distance farthest vertex - centroid
	Sphere(f32),
	//AABB(CoordsRange)
}

pub fn get_ray(meshes: &Vec<Mesh>, centroid: &Point3::<f32>) -> f32 {
    let mut ray = 0.0;
    for mesh in meshes {
        for v in &mesh.vertices {
            let d = distance(&Point3::<f32>::from(v.position), centroid);
            if d < ray {
                ray = d;
            }
        }
    } 
    ray
}

       
pub fn distance(p1: &Point3::<f32>, p2: &Point3::<f32>) -> f32 {
   let d = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt(); 
   d
}

pub fn detect_and_react(entities: &mut HashMap<Role,Entity>) {

    let mut detections: Vec<(Role, Role)> = vec![];

    for (role_a, entity_a) in entities.iter() {
        // for each entity we have to check for any other entity if theres a collision
         for (role_b, entity_b) in entities.iter() {
             // if this is true -> the models are close enough to cause a collition but
             // we are not sure
             if role_b != role_a {
                 if spheres_detection(&entity_a.model.physics, &entity_b.model.physics) {
                    detections.push((role_a.clone(), role_b.clone())); 
                 }
             }
         }       
    }

    let mut collisions: Vec<(Role, f32, Vector3::<f32>)> = vec![];

    for (role_a, role_b) in detections.iter_mut() {
       for mesh_a in entities.get(role_a).unwrap().model.meshes.iter(){                             
           for mesh_b in entities.get(role_b).unwrap().model.meshes.iter() {
                let (factor, normal) = SAT_detection(mesh_a, mesh_b);
                collisions.push((role_a.clone(), factor, normal));
           }
       } 
    }

    for (role_a, factor, normal) in collisions.into_iter() {
        SAT_response(&mut entities.get_mut(&role_a).unwrap().model, factor.clone(), normal.clone());
    }
}   

pub fn SAT_detection(a: &Mesh, b: &Mesh) -> (f32, Vector3::<f32>) {

   let mut separation = F32_MIN;
   let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0); 
   let mut normal_len = 0.0;
   for va in a.vertices.iter() {
        
       let va_normal = cgmath::Vector3{
            x: va.normal[0],
            y: va.normal[1],
            z: va.normal[2]
        }; 

        let mut min_sep = F32_MAX;
        
        for vb in b.vertices.iter() {
            let vdifference = Vector3::<f32> {
                x: vb.position[0] - va.position[0],
                y: vb.position[1] - va.position[1],
                z: vb.position[2] - va.position[2], 
            };
            let dot =  cgmath::dot(vdifference, Vector3::<f32>::from(va.normal));
            if dot < min_sep {
                min_sep = dot;
            }
            let va_v = Vector3::<f32>::new(va.position[0], va.position[1], va.position[2]);
            let vb_v = Vector3::<f32>::new(vb.position[0], vb.position[1], vb.position[2]);

            let n = calculate_normal(&va_v, &vb_v);
            let n_len = (n.x.powi(2) + n.y.powi(2) + n.z.powi(2)).sqrt();
            if  n_len < normal_len {
                normal = n;
                normal_len = n_len;
            }
        }

        if min_sep >  separation {
            separation = min_sep;
        }

   }

   return (separation, normal);
}

pub fn calculate_normal(p1: &Vector3::<f32>, p2: &Vector3::<f32>) -> Vector3::<f32> {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let dz = p2.z - p1.z;

    let ux = if dz.abs() > f32::EPSILON {
        0.0
    } else {
        1.0
    };
    let uy = 1.0;
    let uz = if dz.abs() > f32::EPSILON {
        -dx / dz
    } else {
        0.0
    };

    let nx = dy * uz - dz * uy;
    let ny = dz * ux - dx * uz;
    let nz = dx * uy - dy * ux;

    let magnitude = (nx * nx + ny * ny + nz * nz).sqrt();
    let nx_normalized = nx / magnitude;
    let ny_normalized = ny / magnitude;
    let nz_normalized = nz / magnitude;

    Vector3::new(nx_normalized, ny_normalized, nz_normalized)
}

// I HAVE SERIOUS DOUBTS ABOUT THIS CODE
pub fn SAT_response(model: &mut Model, factor: f32, normal: Vector3::<f32>) {
    
    for mesh in model.meshes.iter_mut() {
        let mut indices: Vec<(usize, f32, f32, f32)> = vec![];
        let mut counter = 0;
        for mut vertex in &mut mesh.vertices.iter() {
            
            let vertex_vec = Vector3::<f32> {
                x: vertex.position[0],
                y: vertex.position[1],
                z: vertex.position[2]
            };

            // this should also depend on the speed and the mass of the model
            let vec = cgmath::Vector3::cross(vertex_vec,(normal / factor / model.physics.mass));
            
                        indices.push((counter.clone() as usize, vec.x, vec.y, vec.z));
            counter += 1;
        }
        
        for (i, x, y, z) in indices {
            mesh.vertices[i].position[0] = x;
            mesh.vertices[i].position[1] = y;
            mesh.vertices[i].position[2] = z;
        }
    }
}



pub fn spheres_detection(a: &PhysicalBody, b: &PhysicalBody) -> bool {
    match (&a.collition_box, &b.collition_box) {
        (&CollitionBox::Sphere(a_ray), &CollitionBox::Sphere(b_ray)) => {
            let distance = distance(&a.centroid, &b.centroid);

            if distance > (a_ray + b_ray) {
                return true;
            } else {
                return false;
            }
        } 
    }
}

