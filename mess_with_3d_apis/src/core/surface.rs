use wgpu::{
    Surface,
    SurfaceConfiguration,
    TextureUsages,
    Adapter,
};

use winit::window::Window;

pub struct SurfaceData {
    pub surface: Surface,
    pub configuration: SurfaceConfiguration
}

impl SurfaceData {
    
    pub fn create_configuration(surface: &Surface, adapter: &Adapter, window: &Window) -> SurfaceConfiguration {
       let surface_capabilities = surface.get_capabilities(adapter); 
       let format = surface_capabilities.formats[0];
       let present_mode = surface_capabilities.present_modes[0];
       let alpha_mode = surface_capabilities.alpha_modes[0];
       let configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window.inner_size().width,
            height: window.inner_size().height, 
            present_mode,
            alpha_mode,
            view_formats: vec![]
       };
       configuration
    }
}

