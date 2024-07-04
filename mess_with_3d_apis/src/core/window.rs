use winit::{
    window::{
        Icon,
    },

};

// AUDIO
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
//
use crate::errors::*;
use crate::WTux;
use image::{
    DynamicImage
};
use crate::errors::*;
use crate::core::objmodel::DrawModel;
use rand::seq::SliceRandom;
use rand::prelude::*;
use cgmath::*; 
 use winit::{
    event_loop::{
        EventLoop,
        ControlFlow
    },
    dpi::PhysicalSize,
    window::Window,
    event::{
        Event,
        WindowEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState
    }
};
use crate::core::light::DrawLight;

use anyhow::{
    Result,
    Error
};

use crate::{
    errors::*,
    core::{
		interaction::{
			Action,
			Interaction,
			PressState
		},
        surface::{
            SurfaceData
        },
        texture::{
            TextureData,
        },
        instance,
        instance::{
            InstanceData,
            InstanceRaw
        },
        camera::{
            CameraData,
            Camera,
            CameraUniform
        },
        controller::Controller,
        objmodel,
        vertex::{
			ModelVertex,
		},
        buffer,
        bindgroup,
        bindgroup::{
            BGLEntry
        },
        pipeline,
        light::{
			LightData,
			LightUniform
		},
		collition::{
			CollitionBox
		}
    }
};

use wgpu::{
    Instance,
    Adapter,
    Device,
    Queue,
    Surface,
    RequestAdapterOptions,
    PowerPreference,
    RequestDeviceError,
    DeviceDescriptor,
    Limits,
    Features,
    InstanceDescriptor,
    ShaderModuleDescriptor,
    ShaderModule,
    Backends,
    Dx12Compiler,
    BufferUsages,
    ShaderStages
};

use log::info;

pub struct WindowData<'a> {
    pub window: Window,
    pub title: &'a str,
    pub inner_size: PhysicalSize<u32>
}

impl <'a>WindowData<'a> {

    pub fn configure_window(window_data: &mut WindowData, icon: Option<Icon>) {
        window_data.window.set_title(window_data.title);
        window_data.window.set_window_icon(icon);
        window_data.window.set_inner_size(window_data.inner_size);
        window_data.window.set_cursor_visible(false);
    }

    pub fn new_icon(name: &str) -> Result<Icon, anyhow::Error> {
        let image: DynamicImage = match image::open(name) {
            Ok(i) => i,
            Err(e) => bail!(ImageError::OpenImageError(e))
        };
        let rgb_icon: Vec<u8> = image.to_rgba8().into_raw();
        let icon = Icon::from_rgba(rgb_icon, image.width(), image.height());
        match icon {
            Ok(i) => Ok(i),
            Err(e) => bail!(WindowError::IconError(e))
        }
    }
}

impl<'a> WTux<'a> {
	pub fn handle_events(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow, audio_handler: &rodio::OutputStreamHandle) { //file: BufReader<std::fs::File>) {
	
		match event {
            WindowEvent::CloseRequested| WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
				..
            } => *control_flow = ControlFlow::Exit,

            // when pressing SPACE, switch to LINE mode
            WindowEvent::KeyboardInput {
                input: KeyboardInput { 
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::X),
					..
				},
                ..
            } => {
                self.update_pipeline();
            }
            
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
					state: ElementState::Pressed,
					virtual_keycode,
					..
				},
                ..
            } => {
				for mut interaction in &mut self.interactions {
					if interaction.key == virtual_keycode.unwrap() {
						
						if interaction.should_we_interact(&mut self.obj_player.meshes[0], &mut self.obj_model.meshes[0]) {
							match interaction.action {
								Action::Print => {
									println!("abbiamo interagito");
								},
								Action::RotateMonkey => {
									
										//MAKE A SOUND
										
										if interaction.state == PressState::Pressed {
											
												let file = BufReader::new(File::open("flash.wav").unwrap());
												let source = Decoder::new(file).unwrap();
											// Play the sound directly on the device
											audio_handler.play_raw(source.convert_samples());
											
											
											interaction.state = PressState::Continuous
										}
										
										//
										
									for mut mesh in &mut self.obj_model.meshes {
										for mut vertex in &mut mesh.raw_vertices {
											let old_position: cgmath::Vector3<_> = vertex.position.into();
											vertex.position = (cgmath::Quaternion::from_axis_angle((1.0, 1.0, 1.0).into(), cgmath::Deg(1.0)) * old_position).into();
										}
														
										mesh.vertex_buffer = buffer::create_buffer(
											&self.device,
											mesh.raw_vertices.clone(),
											BufferUsages::VERTEX
										);
														
										CollitionBox::update_from_vertices(&mut mesh);
									}
								}
							}
						}
					} else {
						if interaction.state == PressState::Continuous {
							interaction.state = PressState::Pressed;
						}
					}
				}
								
				// for each mesh we check if the eye is in the mesh's box ranges -> eventually we move it to
				// it's last position by checking which was the last value that matched the range
				println!("THE PLAYER CENTER IS {:?}", &self.obj_player.meshes[0].collition_box.centroid);
				//println!("{:?}", self.obj_plane.meshes.len());
				//for mesh in self.obj_plane.meshes.iter() {
                    //mesh.meshbox.check_point(&mut state.camera_data.camera.eye, &mut state.camera_data.camera.check);
                    //mesh.meshbox.check_point(&mut state.camera_data.camera.eye, &mut state.camera_data.camera.check);
                    //mesh.meshbox.check_point(&mut state.camera_data.camera.eye, &mut state.camera_data.camera.check);
                    //mesh.meshbox.check_point(&mut state.obj_player.meshes[0].center, &mut state.obj_player.meshes[0].check);
                //}
				
				// We update the controller booleans on WASD
                self.controller.update_controller(virtual_keycode.clone());
                        
                // we update the camera eye position and also the player position
                self.controller.update_on_movement(&self.device, &mut self.camera_data, &mut self.obj_player);
                               
                // then we check if the eye is inside a mesh and eventually move it back to the last position
                //MeshBox::avoid_center_collition(&mut state.obj_player.meshes[0], &state.obj_plane.meshes[0]);
                CollitionBox::update_from_vertices(&mut self.obj_player.meshes[0]);
                
                self.obj_player.meshes[0].collition_box.detect_collition(&mut self.obj_model.meshes[0].collition_box);
                // the view gets updated with the new eye and target coords
                self.camera_data.camera_uniform.update_view_proj(&self.camera_data.camera);
                                
                self.queue.write_buffer(
					&self.camera_data.buffer, 0, bytemuck::cast_slice( &[self.camera_data.camera_uniform])
                );
                                
                // we request the window to re-render in order to see the changes
                self.window_data.window.request_redraw();
                
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }

            WindowEvent::CursorMoved{
                position,
                ..
            } => {
                //state.camera_data.camera.target = cgmath::Point3{x: 0.0, y: 0.0, z: 0.0};
                //state.camera_data.camera.target = cgmath::Point3{x: 0.0, y: 0.0, z: 3.0};
                let pixel_unit_conv: f64 = (self.camera_data.camera.aspect * 100.0 ) as f64;
                //state.camera_data.camera.target.x =  
                //( position.x / pixel_unit_conv  - (state.window_data.window.inner_size().width / pixel_unit_conv as u32 / 2)as f64 ) as f32;

                //state.camera_data.camera.target.y = 
                //(-position.y / pixel_unit_conv + (state.window_data.window.inner_size().height / pixel_unit_conv as u32 / 2) as f64 ) as f32;

                //state.camera_data.camera.mid.x =  
                //( position.x / pixel_unit_conv  - (state.window_data.window.inner_size().width / pixel_unit_conv as u32 / 2)as f64 ) as f32;

                //state.camera_data.camera.mid.y =  
                //( position.y / pixel_unit_conv  - (state.window_data.window.inner_size().width / pixel_unit_conv as u32 / 2)as f64 ) as f32;
			
                self.window_data.window.request_redraw();

                },
                
            _ => {}
        }
	}
}
