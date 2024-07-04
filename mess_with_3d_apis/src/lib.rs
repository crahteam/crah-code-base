pub mod core;
pub mod errors;
pub mod utils;
use image::{
	GenericImageView,
	Pixel
	
};
// AUDIO
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
//
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
#[macro_use]
extern crate anyhow;

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
        window::{
            WindowData
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
		},
		image::{
			ImageData,
			ImageDataLuma
		},
		terrain::{
			Terrain
		},
		star::{
			Stars
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
    ShaderStages,
    PolygonMode
};

use log::info;

pub struct WTuxConfiguration<'a> {
    label: Option<&'a str>
}


pub struct WTux<'a> {
    configuration: WTuxConfiguration<'a>,
    window_data: WindowData<'a>,
    device: Device,
    queue: Queue,
    shader: ShaderModule,
    surface_data: SurfaceData,
    camera_data: CameraData,
    controller: Controller,
    render_pipeline_layout: wgpu::PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
    obj_model: objmodel::Model<'a>,
    obj_grass: objmodel::Model<'a>,
    instance_data: InstanceData,
    depth_texture: TextureData,
    fill_mode_on: bool,
    //obj_plane: objmodel::Model<'a>,
    terrain: Terrain<'a>,
    obj_player: objmodel::Model<'a>,
    light_data: LightData,
    interactions: Vec<Interaction<'a>>,
    stars: Stars<'a>
   // field_limit: FieldLimit
}

impl<'a> WTux<'a> {

    pub fn request_adapter(instance: &Instance, surface: &Surface) -> Result<Adapter, AdapterError> {
       let option: Option<Adapter> = pollster::block_on( instance.request_adapter(
               &RequestAdapterOptions {
                   power_preference: PowerPreference::HighPerformance,
                   force_fallback_adapter: false,
                   compatible_surface: Some(surface)
               } 
       ));
       option.ok_or(AdapterError())
    }

    pub fn request_device_queue(adapter: &Adapter) -> Result<(Device, Queue), DeviceQueueError> {
       let result: Result<(Device, Queue), RequestDeviceError> = pollster::block_on(
            adapter.request_device(
                &DeviceDescriptor {
                    label: Some("Device Request"),
                    features: Features::POLYGON_MODE_LINE,
                    limits: Limits::default()
                },
                None
                )
           );

       match result {
           Ok(dq) => Ok(dq),
           Err(e) => Err(DeviceQueueError(e))
       }
    }

    pub fn create_shader(device: &Device, label: Option<&str>) -> ShaderModule {
        device.create_shader_module(ShaderModuleDescriptor{
            label: label,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl")))
        })
    }

    pub fn new(window_data: WindowData<'a>) -> Result<WTux<'a>, anyhow::Error> {

       env_logger::init();
        
        let instance = Instance::new(
            InstanceDescriptor {
                backends: Backends::all(),
                dx12_shader_compiler: Dx12Compiler::default()
            }
        );

       info!("The instance has been created");

       let surface = unsafe {
            match instance.create_surface(&window_data.window) {
                Ok(s) => s,
                Err(e) => bail!(SurfaceError::CreateSurfaceError(e))
            }
       };

       info!("The surface has been created");

       let adapter = Self::request_adapter(&instance, &surface)?;

       info!("An adapter was found");

       let (device, queue) = Self::request_device_queue(&adapter)?;
       
       info!("device and queue have been created");

       let surface_conf = SurfaceData::create_configuration(&surface, &adapter, &window_data.window);

       info!("Surface configuration loaded");
       
       surface.configure(&device, &surface_conf);

       info!("The surface has been configured");

       let shader = Self::create_shader(&device, Some("Shader module"));

       info!("Shader file read");

       // camera

       let mut camera = Camera::default(&surface_conf);

       let controller = Controller::default();

       info!("Camera and Controller created");

       let mut camera_uniform = CameraUniform::new();

       camera_uniform.update_view_proj(&camera);

       info!("Camera Uniform updated");

       let camera_buffer = buffer::create_buffer(
           &device,
           vec![camera_uniform],
           BufferUsages::UNIFORM | BufferUsages::COPY_DST
       ); 

       info!("Camera Buffer created");

       let camera_bgl = bindgroup::create_bind_group_layout(
           &device,
           &[
                bgl_entry!(0, ShaderStages::VERTEX, BGLEntry::Buffer)
           ],
           Some("Camera BGL")
        );

       info!("camera BGL created");

       let camera_bg = bindgroup::create_bind_group(
            &device,
            &camera_bgl,
            &[
                bg_entry!(0, camera_buffer.as_entire_binding())
            ],
            Some("Camera BG")
        );
        
       info!("camera BG created: contains the camera buffer");

       // Instance
        const NUM_INSTANCES_PER_ROW: u32 = 100; 
        const SPACE_BETWEEN: f32 = 0.1;
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    //let mut rng = rand::thread_rng();
                    //let mut x: Vec<i32> = (-2000..2000).collect();
                    //let mut z: Vec<i32> = (-2000..2000).collect();

                    //x.shuffle(&mut rng);
                    //z.shuffle(&mut rng);

                    //let x: f32 = x[0] as f32 / 100.;
                    //let z: f32 = z[0] as f32 / 100.;

                    let position = cgmath::Vector3 { x, y: -6.0, z};

                    let rotation = if position.is_zero() {
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0))
                    };

                    instance::Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        
        let raw_instances_vec = instances.iter().map(instance::Instance::to_raw).collect::<Vec<_>>();

        let instance_buffer = buffer::create_buffer(
            &device,
            raw_instances_vec,
            BufferUsages::VERTEX
        );

        info!("instance buffer created");

        // Model Loading
       
        let texture_bgl = bindgroup::create_bind_group_layout(
            &device,
            &[
                bgl_entry!(0, ShaderStages::FRAGMENT, BGLEntry::Texture),
                bgl_entry!(1, ShaderStages::FRAGMENT, BGLEntry::Sampler)
            ],
            Some("Texture BGL")
        );

		let light_uniform = LightUniform {
			position: [0.0, 10.0, 0.0],
			_padding: 0,
			color: [1.0, 1.0, 1.0],
			_padding2: 0,
		};

		let light_buffer = buffer::create_buffer(
			&device,
			vec![light_uniform],
			BufferUsages::UNIFORM | BufferUsages::COPY_DST
		);

		let light_bgl = bindgroup::create_bind_group_layout(
			&device,
			&[
				bgl_entry!(0, ShaderStages::FRAGMENT | ShaderStages::VERTEX, BGLEntry::Buffer)
			],
			Some("light bgl")
		);
		
		let mut light_bg = bindgroup::create_bind_group(
			&device,
			&light_bgl,
			&[
				bg_entry!(0, light_buffer.as_entire_binding())
			],
			Some("light bg")
		);
		
        let mut obj_model = pollster::block_on(objmodel::Model::load_obj_model(&device, &queue, &texture_bgl, "cube.obj")).unwrap();
        
        let mut obj_plane = pollster::block_on(objmodel::Model::load_obj_model(&device, &queue, &texture_bgl, "plane.obj")).unwrap();

		let mut obj_player = pollster::block_on(objmodel::Model::load_obj_model(&device, &queue, &texture_bgl, "player.obj")).unwrap();
		
		let mut obj_grass = pollster::block_on(objmodel::Model::load_obj_model(&device, &queue, &texture_bgl, "grass_low_poly.obj")).unwrap();
		
		println!("CENTRO PLAYER {:?}", obj_player.collition_box.centroid);
		
		//{
		//	for mut vertex in <&mut Vec<ModelVertex> as Into<&mut Vec<ModelVertex>>>::into(&mut obj_player.meshes[0].raw_vertices) {
		//		let mut vertex: &mut ModelVertex = vertex;
		//		vertex.position[2] += 5.0;
		//	}
		//	
		//	obj_player.meshes[0].vertex_buffer = buffer::create_buffer(
		//		&device,
		//		obj_player.meshes[0].raw_vertices.clone(),
		//		BufferUsages::VERTEX
		//	);
		//	
		//	CollitionBox::update_from_vertices(&mut obj_player.meshes[0]);
        //}
        
		// terrain generation
		
		let height_map_bytes: Vec<u8> = std::fs::read("height_map.jpg").unwrap();
		
		let height_map: ImageDataLuma = luma_from_bytes!(&height_map_bytes);
		
		let terrain = Terrain::new(&device, obj_plane, height_map);
		
        info!("obj model loaded properly");

        let depth_texture = TextureData::create_depth_texture(&device, &surface_conf, "depth texture");

        info!("depth texture created");

        let render_pipeline_layout = pipeline::create_pipeline_layout(
            &device,
            &[&texture_bgl, &camera_bgl, &light_bgl]
        );

        info!("render pipeline layout created");

       
        let render_pipeline = pipeline::create_pipeline(
            &device,
            &render_pipeline_layout,
            &shader,
            ("vs_main", "fs_main"),
            &surface_conf,
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            Some(TextureData::DEPTH_FORMAT),
            wgpu::PolygonMode::Fill,
            Some("Render Pipeline with ModelVertex and InstanceRaw vert buf layout")
        );

		let light_render_pipeline = {
			
			let layout = pipeline::create_pipeline_layout(
				&device,
				&[
					&camera_bgl, &light_bgl
				]
			);
			
			let lshader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
				label: Some("Light Shader"),
				source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
			});
			
			pipeline::create_pipeline(
				&device,
				&layout,
				&lshader,
				("vs_main", "fs_main"),
				&surface_conf,
				&[ ModelVertex::desc()],
				Some(TextureData::DEPTH_FORMAT),
				wgpu::PolygonMode::Fill,
				Some("light render pipeline")
			)	
		};
		
        info!("render pipeline created");

        let surface_data = SurfaceData {
           surface,
           configuration: surface_conf
        };
        
        camera.target = obj_player.meshes[0].collition_box.centroid;
        
        let camera_data = CameraData {
           camera,
           camera_uniform,
           buffer: camera_buffer,
           bind_group: camera_bg
        };

        let instance_data = InstanceData {
            instances,
            instance_buffer 
        };

        let controller = Controller::default();
		
		let light_data = LightData {
			light_uniform,
			light_buffer,
			light_bg,
			light_bgl,
			render_pipeline: light_render_pipeline
		};
		
		
		let interactions: Vec<Interaction> = vec![
			Interaction {
					mesh: "player mesh",
					target_mesh: "the monkey",
					key: VirtualKeyCode::E,
					action: Action::RotateMonkey,
					ray: 3.0,
					state: PressState::Pressed
			},
			Interaction {
					mesh: "player mesh",
					target_mesh: "the monkey",
					key: VirtualKeyCode::F,
					action: Action::Print,
					ray: 3.0,
					state: PressState::Pressed
			},
		];
		
		
		// STARS RENDERING
		let mut star_model = pollster::block_on(objmodel::Model::load_obj_model(&device, &queue, &texture_bgl, "star.obj")).unwrap();
		let stars_pipeline_layout = pipeline::create_pipeline_layout(
          &device,
           &[&texture_bgl, &camera_bgl, &light_data.light_bgl]
        );
         
        let stars_shader = device.create_shader_module(ShaderModuleDescriptor{
            label: Some("shader normale ma per la stella lol"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl")))
        });
        
		let stars_pipeline = pipeline::create_pipeline(
			&device,
			&stars_pipeline_layout,
			&stars_shader,
			("vs_main","fs_stars"),
			&surface_data.configuration,
			&[
				ModelVertex::desc(),
				InstanceRaw::desc()
			],
			Some(TextureData::DEPTH_FORMAT),
			PolygonMode::Fill,
			Some("stars pipeline")
		);
		
		const NUM_STARS_PER_ROW: u32 = 20; 
        const SPACE_BETWEEN_STARS: f32 = 0.1;
        
        // Data to shuffle in the instance to get random values. 
        //let mut star_x: Vec<i32> = (-2000..2000).collect();
        //let mut star_z: Vec<i32> = (-2000..2000).collect();
		//let mut star_y: Vec<i32> = (-2000..2000).collect();
		// instaead of recreating a vec each time, we just shuffle it again.
		
		let world_coords = Vector3 { x: 0.0, y: 0.0, z: 0.0,};
		let ray: f32 = 50.0;
		
		let stars_instances = (0..NUM_STARS_PER_ROW).flat_map(|z| {
                (0..NUM_STARS_PER_ROW).map(move |x| {
                    let mut position: Vector3::<f32>;
                    let mut rotation: Quaternion::<f32>;
                    
                    loop {
						
						let mut rng = rand::thread_rng();
						
						let mut star_x: Vec<i32> = (-100..100).collect();
						let mut star_z: Vec<i32> = (-100..100).collect();
						let mut star_y: Vec<i32> = (-100..100).collect();
						// mescola i vettori
						star_x.shuffle(&mut rng);
						star_z.shuffle(&mut rng);
						star_y.shuffle(&mut rng);
						
						let x: f32 = star_x[0] as f32;
						let y: f32 = star_y[0] as f32;
						let z: f32 = star_z[0] as f32;
						
						position = cgmath::Vector3 { x, y, z};

						// keeping it d times d makes it more performant
						let distance_2nd = 
							(world_coords.x - x).powf(2.0) +
							(world_coords.y - y).powf(2.0) +
							(world_coords.z - z).powf(2.0);
						
						if distance_2nd < ray.powf(2.0) {
							continue;
						}
						
						rotation = if position.is_zero() {
							cgmath::Quaternion::from_axis_angle(
								cgmath::Vector3::unit_z(),
								cgmath::Deg(0.0),
							)
						} else {
							cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0))
						};

						break;
					}
					
					instance::Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        
        let raw_stars_vec = stars_instances.iter().map(instance::Instance::to_raw).collect::<Vec<_>>();

        let star_instance_buffer = buffer::create_buffer(
            &device,
            raw_stars_vec,
            BufferUsages::VERTEX
        );
        
        let star_instance_data = InstanceData {
            instances: stars_instances,
            instance_buffer: star_instance_buffer 
        };
        
        let stars = Stars {
			model: star_model,
			instance_data: star_instance_data,
			pipeline: stars_pipeline,
			ray: ray
		};
		
        Ok(WTux {
           configuration: WTuxConfiguration{
                label: Some("engine")
           },
           window_data: window_data,
           device,
           queue,
           shader,
           surface_data,
           camera_data,
           controller,
           render_pipeline_layout,
           render_pipeline,
           obj_model,
           obj_player,
           obj_grass,
           instance_data,
           depth_texture,
           fill_mode_on: true,
           light_data,
           interactions,
           stars,
           terrain
        })
    }
    
    fn render(&mut self) -> Result<(), Error> {
		
		println!("CENTRO PLAYER {:?}", self.obj_model.meshes[0].collition_box.centroid);
		println!("CENTRO PLAYER {:?}", self.obj_model.collition_box.centroid);
		println!("CENTRO PLAYER {:?}", self.obj_player.collition_box.centroid);
        let old_position: cgmath::Vector3<_> = self.light_data.light_uniform.position.into();
		self.light_data.light_uniform.position =
			(cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0)) * old_position)
				.into();
		self.queue.write_buffer(&self.light_data.light_buffer, 0, bytemuck::cast_slice(&[self.light_data.light_uniform]));

        let output = self.surface_data.surface.get_current_texture()?;
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &self.depth_texture.texture_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        }),
                    });

					render_pass.set_pipeline(&self.light_data.render_pipeline); // NEW!
					render_pass.draw_light_model(
						&self.obj_model,
						&self.camera_data.bind_group,
						&self.light_data.light_bg,
					); // NEW!
					
                    render_pass.set_vertex_buffer(1, self.instance_data.instance_buffer.slice(..));
                    render_pass.set_pipeline(&self.render_pipeline);
            
                    //render_pass.draw_model_instanced(
                    //    &self.obj_grass,
                    //    0..self.instance_data.instances.len() as u32,
                     //   &self.camera_data.bind_group,
                    //    &mut self.camera_data.camera,
                    //    &self.light_data.light_bg
                    //);
                    render_pass.draw_model(
                        &self.obj_model,
                        &self.camera_data.bind_group,
                        &mut self.camera_data.camera,
                        &self.light_data.light_bg
                    );
                    render_pass.draw_model(
                        &self.terrain.plane,
                        &self.camera_data.bind_group,
                        &mut self.camera_data.camera,
                        & self.light_data.light_bg
                    );
                    render_pass.draw_model(
                        &self.obj_player,
                        &self.camera_data.bind_group,
                        &mut self.camera_data.camera,
                        &self.light_data.light_bg
                    );
                   
                    render_pass.set_vertex_buffer(1, self.stars.instance_data.instance_buffer.slice(..));
                    render_pass.set_pipeline(&self.stars.pipeline);
                    render_pass.draw_model_instanced(
						&self.stars.model,
						0..self.stars.instance_data.instances.len() as u32,
						&self.camera_data.bind_group,
                        &mut self.camera_data.camera,
                        &self.light_data.light_bg
					);
                    // DRAW THE STARS
                    
                    
                    
                    //for mesh in &self.stars.model.meshes {
					//	render_pass.set_vertex_buffer(0,mesh.vertex_buffer.slice(..));
					//	render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
					//	render_pass.set_bind_group(0, &self.stars.model.materials[mesh.material].texture_data.bind_group.as_ref().unwrap(), &[]);
					//	render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.instance_data.instances.len() as u32)
					//}
                    
                    // come faccio a renderizzzare qualcosa usando un altra shader?
                    // devo usare una pipeline diversa
                    
                    // set nuova pipeline
                    
                    //self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
					//self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
					//self.set_bind_group(0, &material.texture_data.bind_group.as_ref().unwrap(), &[]);
					//self.set_bind_group(1, camera_bind_group, &[]);
					//self.set_bind_group(2, light_bind_group, &[]);
					//self.draw_indexed(0..mesh.num_elements, 0, instances);
                }

                self.queue.submit(std::iter::once(encoder.finish()));
                output.present();

                Ok(())
            }


    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            if new_size.width > 0 && new_size.height > 0{
                self.camera_data.camera.aspect = self.surface_data.configuration.width as f32 / self.surface_data.configuration.height as f32;
                self.surface_data.configuration.width = new_size.width;
                self.surface_data.configuration.height = new_size.height;
                self.surface_data.surface.configure(&self.device, &self.surface_data.configuration);
                self.depth_texture =
                    TextureData::create_depth_texture(&self.device, &self.surface_data.configuration, "depth_texture");
            }
        }


    pub fn update_pipeline(&mut self) {
        if self.fill_mode_on {
            let new_pipeline = pipeline::create_pipeline(
                &self.device,
                &self.render_pipeline_layout,
                &self.shader,
                ("vs_main", "fs_main"),
                &self.surface_data.configuration,
                &[
                    ModelVertex::desc(),
                    InstanceRaw::desc()
                ],
                Some(TextureData::DEPTH_FORMAT),
                wgpu::PolygonMode::Line,
                Some("Render Pipeline with ModelVertex and InstanceRaw vert buf layout")
            );
            self.render_pipeline = new_pipeline;
            self.fill_mode_on = false;
        } else {
            let new_pipeline = pipeline::create_pipeline(
                &self.device,
                &self.render_pipeline_layout,
                &self.shader,
                ("vs_main", "fs_main"),
                &self.surface_data.configuration,
                &[
                    ModelVertex::desc(),
                    InstanceRaw::desc()
                ],
                Some(TextureData::DEPTH_FORMAT),
                wgpu::PolygonMode::Fill,
                Some("Render Pipeline with ModelVertex and InstanceRaw vert buf layout")
            );
            self.render_pipeline = new_pipeline;
            self.fill_mode_on = true;
        }
    }
    pub async fn run() -> Result<(), Error> {
        let event_loop = EventLoop::new(); 
        let window = match Window::new(&event_loop) {
            Ok(w) => w,
            Err(e) => bail!(WindowError::CreateWindowError(e))
        };
        let icon = WindowData::new_icon("icona.png")?; 
        let inner_size = PhysicalSize {
            width: 700,
            height: 700
        };

        let mut window_data = WindowData{
            window,
            title: "titolo",
            inner_size
        };

        WindowData::configure_window(&mut window_data, Some(icon));

        let mut state = WTux::new(window_data)?;

		let mut angle: f32 = 0.0;
		let mut grass_speed: f32 = 0.15;
		let grass_angle = 10.0;
		let smooth = 0.00006;
		
		// RODIO
		
		let (_stream, stream_handle) = OutputStream::try_default().unwrap();
		// Load a sound from a file, using a path relative to Cargo.toml
		
		// Decode that sound file into a source
		
											
		//
									
		// se n frame * velocita rotazione = 45, allora cambia verso e resetta il counter
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
           
            match event {
                Event::MainEventsCleared => state.window_data.window.request_redraw(),
                Event::WindowEvent {ref event, window_id} if window_id == state.window_data.window.id() => {
					state.handle_events(event, control_flow, &stream_handle);
				}
                Event::RedrawRequested(window_id) if window_id == state.window_data.window.id() => {
					
					// ERBA
					
					for mut mesh in &mut state.obj_grass.meshes {
						for mut vertex in &mut mesh.raw_vertices {
						
							if (angle) > grass_angle {
								grass_speed = grass_speed - smooth;
								
							}
							
							if (angle) < -grass_angle {
								grass_speed = grass_speed + smooth;
							}
							
							let old_position: cgmath::Vector3<_> = vertex.position.into();
							vertex.position = (cgmath::Quaternion::from_axis_angle((0.00, grass_speed, 0.0).into(), cgmath::Deg(1.0)) * old_position).into();
							vertex.position[0] += 0.00001;
									
						}
														
						mesh.vertex_buffer = buffer::create_buffer(
							&state.device,
							mesh.raw_vertices.clone(),
							BufferUsages::VERTEX
						);
														
					CollitionBox::update_from_vertices(&mut mesh);
				}
					angle += grass_speed;
					state.render();
                },
                _ => {}
            }
            
        });

    }
}
