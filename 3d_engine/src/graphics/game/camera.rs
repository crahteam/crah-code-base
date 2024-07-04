use wgpu::{
    Buffer,
    BindGroup,
    Device,
    BufferUsages
};
use crate::graphics::buffer;
use crate::graphics::game::controller::CameraController;
pub enum CameraMode {
	WorldCamera,
	Player
}

pub struct CameraDescriptor {
    pub position: Point3::<f32>,
    pub yaw: cgmath::Deg::<f32>,
    pub pitch: cgmath::Deg::<f32>,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32
}

pub struct CameraData {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub projection: Projection,
    pub buffer: Buffer,
    //pub bind_group: BindGroup,
    pub controller: Option<CameraController>
}

impl CameraData {
    pub fn new(device: &Device, desc: CameraDescriptor) -> Self {
        let camera = Camera::new(desc.position, desc.yaw, desc.pitch);
        let mut camera_uniform = CameraUniform::new();
        let proj = Projection::new(desc.width, desc.height, cgmath::Deg(desc.fov), desc.znear, desc.zfar);
        camera_uniform.update_view_proj(&camera, &proj);

        let buffer = buffer::create_buffer(device, vec![camera_uniform].clone(), BufferUsages::UNIFORM | BufferUsages::COPY_DST); 
        Self {
            camera,
            camera_uniform,
            projection: proj,
            buffer,
            controller: None
        }
    }
}
use cgmath::*;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use std::f32::consts::FRAC_PI_2;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);



#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>
}

impl Camera {
    pub fn new<
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            Vector3::unit_y(),
        )
    }
}


pub struct Projection {
    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
