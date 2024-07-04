use wgpu::{
    Surface,
    SurfaceConfiguration,
    TextureUsages,
    Adapter,
    Instance
};

use winit::window::Window;
use anyhow::{
    bail,
    Error
};

use crate::errors::graphics::{
    SurfaceError
};

pub struct SurfaceData {
    pub surface: Surface,
    pub configuration: SurfaceConfiguration
}

impl SurfaceData {

    pub unsafe fn new(adapter: &Adapter, instance: &Instance, window: &Window) -> Result<Self, Error> {
        let surface = Self::new_surface(instance, window)?;
        let configuration = Self::new_configuration(&surface, adapter, window);
        
        Ok(Self {
            surface,
            configuration
        })
    }

    pub unsafe fn new_surface(instance: &Instance, window: &Window) -> Result<Surface, Error> {
        match instance.create_surface(window) {
            Ok(s) => Ok(s),
            Err(e) => bail!(SurfaceError::CreateSurfaceError(e))
        }
    }

    pub fn new_configuration(surface: &Surface, adapter: &Adapter, window: &Window) -> SurfaceConfiguration {
		
       let surface_capabilities = surface.get_capabilities(adapter); 
       let format = surface_capabilities.formats[0];
       let present_mode = surface_capabilities.present_modes[0];
       let alpha_mode = surface_capabilities.alpha_modes[0];
       
       SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window.inner_size().width,
            height: window.inner_size().height, 
            present_mode,
            alpha_mode,
            view_formats: vec![]
       }
    }
}
