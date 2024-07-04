use cgmath::{
    Vector3,
    Point3,
    Deg
};
use bytemuck::{
    Pod, Zeroable
};
use wgpu::{
    Buffer,
    BindGroup,
    SurfaceConfiguration
};
use crate::core::{
   vertex::{
        CoordsBool
   } 
};

pub enum CameraStyle {
	FirstPerson,
	ThirdPerson(Deg::<f32>, u32)
}

pub struct CameraData {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub buffer: Buffer,
    pub bind_group: BindGroup
}

pub struct Camera {
    pub eye: Point3::<f32>,
    pub target: Point3::<f32>,
    pub up: Vector3::<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    //pub mid: Point3::<f32>,
    pub check: CoordsBool
}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


impl Camera {
 pub fn default(s_conf: &SurfaceConfiguration) -> Self {
        Camera {
            eye: Point3::<f32> {x: 0.0, y: 0.0, z: 0.0},
            target: Point3::<f32> {x: 0.0, y:0.0, z:0.0},
            up: Vector3::unit_y(),
            aspect: (s_conf.width / s_conf.height) as f32,
            fovy: 45.0,
            znear: 1.0,
            zfar: 100.0,
            check: CoordsBool{
                x: false,
                y: false,
                z: false
            },
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn back_to_field(&mut self, check: (bool, bool), speed: f32) {

        let (x_p, z_p): (bool, bool);
        if &self.eye.x > &0.0 { x_p = true} else { x_p = false};
        if &self.eye.z > &0.0 { z_p = true} else { z_p = false};

        if check.0 == false {
            match x_p {
                true => {self.eye.x -= speed},
                false => {self.eye.x += speed}
            }
        }

        if check.1 == false {
            match z_p {
                true => {self.eye.z -= speed},
                false => {self.eye.z += speed}
            }
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[macro_export]
macro_rules! betw_two{
    ($a:expr, $b:expr, $c:expr) => {{
        match $b > $a && $b < $c {
            true => { true },
            false => { false }
        }}
    }
}


pub struct FieldLimit {
    pub x_range: (f32, f32),
    pub z_range: (f32, f32)
}

impl FieldLimit {

    pub fn check_cam_position(&self, eye_pos: &Point3<f32>) -> (bool, bool) {
        let mut check: (bool, bool) = (
            betw_two!(&self.x_range.0, &eye_pos.x, &self.x_range.1),
            betw_two!(&self.z_range.0, &eye_pos.z, &self.z_range.1)
        );
        check
    }
}



