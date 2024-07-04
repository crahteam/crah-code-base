use winit::{
    event_loop::{
        EventLoop,
        ControlFlow
    },
    dpi::{
        PhysicalSize
    },
    event::{
        VirtualKeyCode,
        Event,
        WindowEvent,
        KeyboardInput,
        ElementState
    }
};
use crate::audio::Audio;
use rodio::{
    OutputStreamHandle,
    OutputStream,
    Decoder
};
use cgmath::Rotation3;
use std::collections::HashMap;
use crate::utils::image::ImageReader;
use crate::graphics::{
    window::WindowDescriptor,
    state::State,
    render,
    bindgroup::{
        bind_group_layout_entry,
        create_bind_group_layout,
        BGLEntry,
        bind_group_entry,
        create_bind_group
    },
    pipeline::{
        create_pipeline,
        create_pipeline_layout
    },
    texture::TextureData,
    buffer,
    game::{
        scene::Scene,
        model::ModelDescriptor,
        camera::{
            CameraDescriptor,
            CameraData,
            Camera
        },
        entity::{
            Entity
        },
        controller::{
            Controller,
            CameraController
        },
        vertex::Vertex,
        world::{
            World,
            terrain::{
                ProceduralTerrain,
                ProceduralTerrainDescriptor
            },
            sky::{
                Sky,
                SkyBackground
            }
        }
    }
};

use crate::physics::{
    movement::{
        Movement,
        BasicDirection
    }
};

use wgpu::{
    ShaderStages,
    include_wgsl,
    ShaderModuleDescriptor,
    RenderPipeline,
    BindGroup,
    BufferUsages
};

pub const DIR: &str = "/home/tux/Desktop/engine/res/";

pub enum AudioSource {
    // should be defined by the user
}

#[derive(PartialEq, Eq, Hash)]
pub enum BGS{
    Camera
}

#[derive(Clone, Debug)]
pub enum Actions {
    // shoud be defined by the player    
    Print,
    ChangeBG,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Role {
	Player,
	NCP(NCP),
    Object(Object),
    World(WorldItem)// This includes Map items and sky stuff like stars and more
}// NOTE None should be added in Pipe so during
// the rendering the first iteration over the entities 
 // will set the pipeline
#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Pipe {
    Default,
    None// the various pipelines
}

// NOTE: for each pipe created in the enum Pipe, there should be added a matching arm
// below with the corresponding code to use. you have access to all the bindgroups stored in the
// scene

#[macro_export]
macro_rules! pipeline_matcher {
    ($r: expr, $e: expr, $s: expr) => {
        {
            match $e.pipeline {
                Pipe::Default => {
                    use crate::user::{
                        Role,
                        BGS
                    };
                    for mesh in $e.model.meshes.iter() {
                        $r.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                        $r.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        $r.set_bind_group(0, $e.model.materials[mesh.material].texture_data.bind_group.as_ref().unwrap(), &[]);
                        //$r.set_vertex_buffer(1, $s.cameras.get(&String::from("Camera")).unwrap().buffer.slice(..));
                        $r.set_bind_group(1, $s.bindgroups.get(&BGS::Camera).unwrap(), &[]);
                        $r.draw_indexed(0..mesh.num_elements, 0, 0..1);
                    }
                    //println!("Default Pipeline");
                },
                Pipe::None => {
                   // println!("None Pipeline!!");
                }
            }
        }
    }
}
pub use pipeline_matcher;



#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WorldItem {
    Stars,
    Trees,
    Terrain
    // Should be specified so they get accessible from the world struct, atleast if theres
    // something we know where to get it
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Object {
    // Objects should be added to the game
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum NCP {
	// NCPs should be added depending to the game
}


// NOTE: this is the RUN function
pub fn game() {
 println!("game runnign");
  let event_loop = EventLoop::new(); 
  let window_desc = WindowDescriptor {
      dir: DIR,
      name: "icon.png",
      icon: Some(()),
      title: "my window",
      fullscreen: false,
      inner_size: PhysicalSize::<u32> {
          width: 1000,
          height: 1000
      }
  };

  let mut state = State::new(&event_loop, window_desc).unwrap(); 
  println!("state created");
  // now there should be the scenes
  
  //let scene: Scene;

  let texture_bgl = create_bind_group_layout(
        &state.device,
        &[
            bind_group_layout_entry!(0, ShaderStages::FRAGMENT, BGLEntry::Texture),
            bind_group_layout_entry!(1, ShaderStages::FRAGMENT, BGLEntry::Sampler)
        ],
        Some("Texture BGL")
  );
    println!("texture bgl fatto");
  let camera_desc = CameraDescriptor {
      position: cgmath::Point3::<f32>::new(0.0, 15.0, 15.0),
      yaw: cgmath::Deg::<f32>(180.0),
      pitch: cgmath::Deg::<f32>(180.0),
      width: state.window_data.inner_size.width,
      height: state.window_data.inner_size.height,
      fov: 45.0, // 0.7 = 45 deg 1.5 = 90 deg
      znear: 0.001,
      zfar: 400.0,
  };

  let mut camera = CameraData::new(&state.device, camera_desc);

    // SMALL CORRECTION TO THE ROLL
//    let rotation: cgmath::Vector3::<f32> = (cgmath::Quaternion::<f32>::from_axis_angle(cgmath::Vector3::<f32>::unit_x().into(), cgmath::Deg::<f32>(90.0))).into();
  let camera_bgl = create_bind_group_layout(
      &state.device,
      &[
        bind_group_layout_entry!(0, ShaderStages::VERTEX, BGLEntry::Buffer)
      ],
      Some("Camera bind group layout")
  );

  let camera_bg = create_bind_group(
      &state.device,
      &camera_bgl,
      &[
            bind_group_entry!(0, camera.buffer.as_entire_binding())
      ],
      Some("camera bind group")
  );

  let camera_controller = CameraController::new(10.0, 0.05);
    
  camera.controller = Some(camera_controller);

  println!("camera fatta");
  
  let player_desc = ModelDescriptor {
       dir: DIR,
       name: "player.obj",
       speed: 100.0,
       mass: 75.0,
       position: cgmath::Point3::<f32>::new(0.0, 0.0, 0.0)
  };

  let player = Entity::spawn(
      &state.device,
      &state.queue,
      &texture_bgl,
      player_desc,
      Pipe::Default  ).unwrap();

    println!("player fatto");

  let pipeline_layout = create_pipeline_layout(
      &state.device,
      &[
        &texture_bgl,
        &camera_bgl
      ]
  );

  let default_pipeline_shader = state.device.create_shader_module(
      ShaderModuleDescriptor {
            label: Some("my default pipeline"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("default.wgsl")))     
      }
  );

  let default_pipeline = create_pipeline(
      &state.device, 
      &pipeline_layout,
      &default_pipeline_shader,
      ("vs_main", "fs_main"),
      &state.surface_data.configuration,
      &[Vertex::desc()],
      Some(TextureData::DEPTH_FORMAT),
      wgpu::PolygonMode::Line,
      Some("Default render pipeline")
   );
    println!("pipeline fatta");

    let sky = Sky {
        background: SkyBackground::Rgba([1.0, 1.0, 1.0]),
        ray: 50.0
    };

    println!("cielo fatta");
    
    let terrain_texture = ImageReader::plain_rgba([0.0, 0.0, 1.0]).unwrap();
    let terrain_desc = ProceduralTerrainDescriptor {
        dir: DIR,
        name: "height_map.png",
        pipe: Pipe::Default,
        max_height: 2.0,
        min_height: -2.0,
        image: terrain_texture
    };

    let terrain = ProceduralTerrain::new(&state.device, &state.queue, terrain_desc, &texture_bgl).unwrap(); 

    println!("Ecco il terreno");
    for mesh in terrain.model.meshes.iter() {
        println!("{:?}", mesh);
    }
    
    println!("terrain fatta");
    let world = World {
       sky,
       //terrain
    };
    println!("prima ancora");
    let mut input_action: HashMap<VirtualKeyCode, Actions> = HashMap::new();
    input_action.insert(VirtualKeyCode::Space, Actions::Print);
    input_action.insert(VirtualKeyCode::X, Actions::ChangeBG);
    input_action.insert(VirtualKeyCode::W, Actions::MoveForward);
    input_action.insert(VirtualKeyCode::A, Actions::MoveLeft);
    input_action.insert(VirtualKeyCode::D, Actions::MoveRight);
    input_action.insert(VirtualKeyCode::S, Actions::MoveBack);

    let controller = Controller {
        input_action,
    };

    println!("prima dell audio");
    let (stream, audio_handler) = OutputStream::try_default().unwrap(); 
    let audios: HashMap<AudioSource, Decoder<std::io::BufReader<std::io::Cursor<String>>>> = HashMap::new();

    let audio = Audio {
        audio_handler,
        audios
    };

    println!("audio fatto");

    let mut entities: HashMap<Role, Entity> = HashMap::new();
    entities.insert(Role::Player, player);
    entities.insert(Role::World(WorldItem::Terrain), terrain);
    
    let mut pipelines: HashMap<Pipe, RenderPipeline> = HashMap::new();
    pipelines.insert(Pipe::Default, default_pipeline);

    let mut bindgroups: HashMap<BGS, BindGroup> = HashMap::new();
    bindgroups.insert(BGS::Camera, camera_bg);

    let mut cameras: HashMap::<String, CameraData> =  HashMap::new();
    cameras.insert(String::from("Camera"), camera);

    let scene = Scene {
        entities,
        pipelines,
        bindgroups,
        controller,
        audio,
        cameras,
        world,
    };

    state.scenes.insert("level1", scene);
    println!("tutto pronto");

    let mut last_render_time = std::time::Instant::now(); 

    event_loop.run(move |event, _, control_flow| {
        //let scene = state.scenes.get_mut("level1").unwrap();
        *control_flow = ControlFlow::Poll;
        //println!("loop");

        //println!("player position: {:?}", &state.scenes.get("level1").unwrap().entities.get(&Role::Player).unwrap().model.physics.centroid);
        //println!("terrain position: {:?}", &state.scenes.get("level1").unwrap().entities.get(&Role::World(WorldItem::Terrain)).unwrap().model.physics.centroid);
        //println!("Camera position: {:?}", &state.scenes.get("level1").unwrap().cameras.get(&String::from("Camera")).unwrap().camera.position);
        match event {
            Event::MainEventsCleared => state.window_data.window.request_redraw(),
            Event::RedrawRequested(_) =>{
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                update(&mut state, dt);
                render::render(&state, "level1").expect("errore nel render");
            },
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
             } => {
               let mut controller = &mut state.scenes.get_mut("level1").unwrap().cameras.get_mut(&String::from("Camera")).unwrap().controller;
               match controller {
                    Some(ref mut c) => {c.process_mouse(delta.0, delta.1);},
                    None => {}
               }
            },
            Event::WindowEvent{event, ..} => {
                match event {

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                        ..
                    } => {
                        for (key, action) in state.scenes.get("level1").unwrap().controller.input_action.clone().iter() {
                            let virtual_key = virtual_keycode.unwrap();
                            if &virtual_key == key {
                                 action_matcher(&mut state, action);
                            } 
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(physical_size);
                        state.scenes.get_mut("level1").unwrap().cameras.get_mut(&"Camera".to_string()).unwrap().projection.aspect = (state.window_data.inner_size.width / state.window_data.inner_size.height) as f32;
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(*new_inner_size);
                    }
                    _ => {}
                }
            }
             _ => {}
        }
    })

}

pub fn action_matcher(state: &mut State, action: &Actions) {
    match action {
        Actions::Print => {
            println!("you just acted");
        },
        Actions::ChangeBG => {
           state.scenes.get_mut("level1").unwrap().world.sky.background = SkyBackground::Rgba([0.0, 1.0, 0.0]);
        },
        Actions::MoveForward => {
           let camera = state.scenes.get_mut("level1").unwrap().cameras.get_mut(&String::from("Camera")).unwrap();
            if let Some(controller) = &mut camera.controller {
                // MOVES THE CAMERA FORWARD
                //let movement = Movement::yaw_movement(camera.camera.yaw, BasicDirection::Forward, controller.speed);
                //movement.move_point(&mut camera.camera.position);
                //controller
                // UPDATES
                controller.amount_forward += controller.speed;
            }
        },
        Actions::MoveBack => {
            let camera = state.scenes.get_mut("level1").unwrap().cameras.get_mut(&String::from("Camera")).unwrap();
                if let Some(controller) = &mut camera.controller {
                   controller.amount_backward += controller.speed;
            }
        },
        Actions::MoveLeft => {
            let camera = state.scenes.get_mut("level1").unwrap().cameras.get_mut(&String::from("Camera")).unwrap();
                if let Some(controller) = &mut camera.controller {
                   controller.amount_left += controller.speed;
            }
        },
        Actions::MoveRight => {
            let camera = state.scenes.get_mut("level1").unwrap().cameras.get_mut(&String::from("Camera")).unwrap();
                if let Some(controller) = &mut camera.controller {
                   controller.amount_right += controller.speed;
            }
        },

        _ => {}
    }

    state.window_data.window.request_redraw();
}




pub fn update(state: &mut State, dt: std::time::Duration) {
    let scene = state.scenes.get_mut("level1").unwrap();
    {
        let camera = scene.cameras.get_mut(&String::from("Camera")).unwrap();
        if let Some(controller) = &mut camera.controller {
           controller.update_camera(&mut camera.camera, dt); 
        //camera.projection = Projection::new(state.window_data.inner_size.width, state.window_data.inner_size.height, cgmath::Deg(camera.camera.fov), camera.camera.znear, camera.camera.zfar);
        camera.camera_uniform.update_view_proj(&camera.camera, &camera.projection);
        camera.buffer = buffer::create_buffer(&state.device, vec![camera.camera_uniform].clone(), BufferUsages::UNIFORM | BufferUsages::COPY_DST); 

        let camera_bgl = create_bind_group_layout(
          &state.device,
          &[
            bind_group_layout_entry!(0, ShaderStages::VERTEX, BGLEntry::Buffer)
          ],
          Some("Camera bind group layout")
        );

        *scene.bindgroups.get_mut(&BGS::Camera).unwrap() = create_bind_group(
              &state.device,
              &camera_bgl,
              &[
                    bind_group_entry!(0, camera.buffer.as_entire_binding())
              ],
              Some("camera bind group")
        );
    
        controller.amount_left = 0.0;
        controller.amount_right = 0.0;
        controller.amount_forward = 0.0;
        controller.amount_backward = 0.0;
        }
    }
}
