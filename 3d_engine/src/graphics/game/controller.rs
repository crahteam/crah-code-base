use winit::event::VirtualKeyCode;
use std::collections::HashMap;
use crate::physics::movement::{
    Movement,
    BasicDirection
};
use crate::user::Actions;
// the controller phylosophy is that for each input there's a corresponding action
// each scene has a controller
pub struct Controller {
	pub input_action: HashMap<VirtualKeyCode, Actions>
}
use std::time::Duration;
use crate::graphics::game::camera::Camera;
impl Controller {
}
use std::f32::consts::FRAC_PI_2;
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

// THE USER SHOULD HANDLE THE CASES WHEN TO UPDATE EACH AMOUNT in THE
// VIRTUALKEYCODE/ACTION HASHMAP ABOVE
#[derive(Debug, Clone)]
pub struct CameraController {
   pub amount_left: f32,
   pub amount_right: f32,
   pub amount_forward: f32,
   pub amount_backward: f32,
   pub amount_up: f32,
   pub amount_down: f32,
   pub rotate_horizontal: f32,
   pub rotate_vertical: f32,
   pub speed: f32,
   pub sensitivity: f32,
}

// in most cases the CAMERA MOVEMENTS SHOULD MATCH THE PLAYER MOVEMENTS
// inside input_action: on pressing W: -> move the camera
impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity,
        }
    }


    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();
        //NOTE: THIS ISNT USING DURATION
        let dt = 1.0;
        // Move forward/backward and left/right
        //let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();

        // NOTE:these vectors are basically the direction
        // let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        //let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();

        // by multiplying the direction by the amount we get the actual thing
        
        // WHAT THESE DOES IS MOVEING THE CAMERA IN XZ values.
        // WE NEVER WANT TO MOVE IT ON THE Y
        let amount_forward = (self.amount_forward - self.amount_backward) * self.speed * dt;
        let amount_right = (self.amount_right - self.amount_left) * self.speed * dt;

        let forward = Movement::yaw_movement(camera.yaw, BasicDirection::Forward, amount_forward);
        let right = Movement::yaw_movement(camera.yaw, BasicDirection::Right, amount_right);

        forward.move_point(&mut camera.position);
        right.move_point(&mut camera.position);

        
        //camera.position += forward * 
        //camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        //
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += cgmath::Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += cgmath::Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -cgmath::Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -cgmath::Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > cgmath::Rad(SAFE_FRAC_PI_2) {
            camera.pitch = cgmath::Rad(SAFE_FRAC_PI_2);
        }

    }

}
