use std::collections::HashMap;
use winit::event_loop::EventLoop;
use crate::graphics::game::scene::Scene;
use crate::graphics::{
    window::{
        WindowData,
        WindowDescriptor
    },
    surface::SurfaceData,
    texture::TextureData
};
use wgpu::{
    Device,
    Queue,
    Features,
    Limits,
    Instance,
    Adapter,
    RequestAdapterOptions,
    PowerPreference,
    InstanceDescriptor,
    Backends,
    DeviceDescriptor,
    Dx12Compiler,
    Surface,
    RequestDeviceError
};
use crate::errors::graphics::{
    AdapterError,
    DeviceError
};
use anyhow::{
    Error,
    bail
};

pub struct State<'a> {
    pub	window_data: WindowData<'a>,
    pub	surface_data: SurfaceData,
    pub	device: Device,
    pub	queue: Queue,
    pub	scenes: HashMap<&'a str, Scene>,
    pub depth_texture: TextureData
}

impl<'a> State<'a> {
    pub fn new<R>(event_loop: &EventLoop<R>, window_desc: WindowDescriptor<'a>) -> Result<Self, Error> {
        
       let window_data = WindowData::new(&event_loop, window_desc)?;
       window_data.configure();
       
       let instance = Instance::new(
            InstanceDescriptor {
                backends: Backends::all(),
                dx12_shader_compiler: Dx12Compiler::default()
            }
        );

        let surface = unsafe {
            SurfaceData::new_surface(&instance, &window_data.window)?
        };

        let adapter = Self::request_adapter(&instance, &surface)?;

        let (device, queue) = Self::request_device(&adapter)?;
        let surface_configuration = SurfaceData::new_configuration(&surface, &adapter, &window_data.window);
        surface.configure(&device, &surface_configuration);
        
        let surface_data = SurfaceData {
            surface,
            configuration: surface_configuration
        };

        let depth_texture = TextureData::new_depth_texture(&device, &surface_data.configuration, "depth_texture");

        Ok(Self {
            window_data,
            surface_data,
            device,
            queue,
            scenes: HashMap::new(),
            depth_texture
        })
    }

    pub fn request_adapter(instance: &Instance, surface: &Surface) -> Result<Adapter, anyhow::Error> {
       let option: Option<Adapter> = pollster::block_on( instance.request_adapter(
               &RequestAdapterOptions {
                   power_preference: PowerPreference::HighPerformance,
                   force_fallback_adapter: false,
                   compatible_surface: Some(surface)
               } 
       ));
       //option.ok_or(bail!(AdapterError))
        match option {
            Some(x) => Ok(x),
            None => bail!(AdapterError)
        }
    }

    pub fn request_device(adapter: &Adapter) -> Result<(Device, Queue), Error> {
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
             Err(e) => bail!(DeviceError(e))
        }
    }



    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            //self.camera_data.camera.aspect = self.surface_data.configuration.width as f32 / self.surface_data.configuration.height as f32;
            self.surface_data.configuration.width = new_size.width;
            self.surface_data.configuration.height = new_size.height;
            self.surface_data.surface.configure(&self.device, &self.surface_data.configuration);
            self.depth_texture = TextureData::new_depth_texture(&self.device, &self.surface_data.configuration, "depth_texture");
        }
    }
}
