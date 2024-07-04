use image::{
    DynamicImage,
    RgbaImage,
    GenericImageView,
    GrayImage
};
use rand::prelude::SliceRandom;
use std::fs::File;
use std::io::Write;
use anyhow::{
    Result,
    Error
};

use crate::{
    errors::*,
    utils::{
        environment::create_file,
    }
};

pub struct ImageDataLuma {
	pub image: DynamicImage,
	pub luma: GrayImage,
	pub size: (u32, u32)
}

#[macro_export]
macro_rules! luma_from_bytes {
	($a: expr) => {
		{
			extern crate image as imag;
			use imag::GenericImageView;

			use crate::core::image::ImageData;

			let image = match imag::load_from_memory($a) {
				Ok(img) => img,
				Err(e) => bail!(ImageError::LoadFromBytesError(e))
			};

			// GrayImage = ImageBuffer<Luma8...>
			let luma = image.to_luma8();
			let size: (u32, u32) = image.dimensions();

			ImageDataLuma {
				image,
				luma,
				size
			}
		}
	}
}
pub struct ImageData {
    pub image: DynamicImage,
    pub rgba: RgbaImage,
    pub size: (u32, u32)
}

impl ImageData {

    pub fn load_plain_rgb(diffuse: [f32; 3]) -> Result<Self, Error> { 

        // we create a temporary file for it
        let mut rgb_str: String = "".to_string();

		// mischia l ordine nei vettori a caso
		//let mut rng = rand::thread_rng();
        //let mut r: Vec<u32> = (0..255).collect();
        //let mut b: Vec<u32> = (0..255).collect();

        //r.shuffle(&mut rng);
        //b.shuffle(&mut rng);
        
		// per cio posso prendere il primo numero
        //let r = r[0];
        //let b = b[0];
        
        // unused for now
        for value in diffuse {
            let rgb_val = to_rgb!(value);
            rgb_str = format!("{} {}", rgb_str, rgb_val);
        }
        
		//rgb_str = format!("{} 255 {} 200 255 200", r, b);
		rgb_str = format!("0 255 0 155 255 155 155 255 155");
        let mut start: String = "P3 1 3 255 ".to_string();

        start.push_str(&rgb_str[..]);

        create_file!("plain_rgb.txt", start.as_bytes());
        
        let file = std::fs::read("plain_rgb.txt")?;

        let image_data = image_from_bytes!(&file);
        
        Ok(image_data)
    }

}

#[macro_export]
macro_rules! to_rgb{
    ($a: expr) => {
        {
            let rgb_val: u32 = ((f32::powf($a, 1.0 / 2.4) * 255. - 0.055 * 1.055 * 255.) / 1.055) as u32;
            rgb_val
        }
    }
}
pub use to_rgb;

#[macro_export]
macro_rules! image_from_bytes{
    ($a: expr) => {
        {
            extern crate image as imag;
            use imag::GenericImageView;

            use crate::core::image::ImageData;

            let image = match imag::load_from_memory($a) {
                Ok(img) => img,
                Err(e) => bail!(ImageError::LoadFromBytesError(e))
            };

            let rgba = image.to_rgba8();
            let size: (u32, u32) = image.dimensions();

            ImageData {
                image,
                rgba,
                size
            }

        } 
    }
}

pub use image_from_bytes;
