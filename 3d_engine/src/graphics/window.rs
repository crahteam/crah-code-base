use winit::{
	window::{
        Window,
        Icon,
    },
	dpi::PhysicalSize,
    event_loop::EventLoop
};
use image::DynamicImage;
use anyhow::{
    Error,
    bail
};

use crate::errors::graphics::{
    WindowError,
    IconError
};

use crate::utils::image::{
    ImageReader
};

pub struct WindowData<'a> {
	pub window: Window,
	pub title: &'a str,
	pub inner_size: PhysicalSize<u32>,
	pub icon: Option<Icon>,
	pub fullscreen: bool,
}

impl <'a> WindowData<'a> {	
	
    pub fn new<T>(event_loop: &EventLoop<T>, desc: WindowDescriptor<'a>) -> Result<Self, Error> {

        let window = match Window::new(&event_loop) {
            Ok(w) => w,
            Err(e) => bail!(WindowError::CreateWindowError(e))
        };

        let icon: Option<Icon>;

        if let Some(()) = desc.icon {
            let icon_img = ImageReader::new_rgba(desc.dir, desc.name)?;
            icon = Some(Self::new_icon(icon_img)?);
        } else {
            icon = None;
        }

        let window_data = Self {
            window,
            icon,
            inner_size: desc.inner_size,
            fullscreen: desc.fullscreen,
            title: desc.title
        };

        Ok(window_data)
    }

	pub fn configure(&self) {
		&self.window.set_title(&self.title);
		&self.window.set_inner_size(self.inner_size.clone());
		&self.window.set_window_icon(self.icon.clone());
	}	
	
	pub fn new_icon(image: ImageReader) -> Result<Icon, anyhow::Error> {
        if let ImageReader::Rgba(img) = image {
            let rgb_img = img.image.to_rgba8().into_raw();
            let icon = Icon::from_rgba(rgb_img, img.size.0, img.size.1);
            match icon {
                Ok(i) => Ok(i),
                Err(e) => bail!(IconError::CreateIconError(e))
            }

        } else {
            bail!(IconError::NotRgba);
        }
	} 
}

pub struct WindowDescriptor<'a> {
   pub dir: &'a str,
   pub name: &'a str,
   pub icon: Option<()>,
   pub title: &'a str,
   pub fullscreen: bool,
   pub inner_size: PhysicalSize<u32>
}
