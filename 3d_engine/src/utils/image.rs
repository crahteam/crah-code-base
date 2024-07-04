use image::{
    GenericImageView,
    DynamicImage
};
use crate::utils::{
    reader::read_to_bytes,
    writer::create_file,
};

use crate::errors::{
    utils::ImageError
};
use anyhow::{
    Error,
    bail
};

use crate::user::DIR;
#[derive(Debug)]
pub struct Image {
    pub image: DynamicImage,
    pub size: (u32, u32)
}

#[derive(Debug)]
pub enum ImageReader {
    Rgba(Image),
    Luma(Image)
}

impl ImageReader {

    pub fn plain_rgba(diffuse: [f32; 3]) -> Result<Self, Error> {
        let mut start = String::from("P3 1 1 255 ");
        let mut rgb = "".to_string();
        for v in diffuse {
            rgb = format!("{} {}", rgb, to_rgb!(v));
        }
        start.push_str(&rgb[..]);
        create_file!("rgb.txt", start.as_bytes());
        Ok(Self::new_rgba("", "rgb.txt")?)
    }

    pub fn new_rgba(dir: &str, name:&str) -> Result<Self, Error> {

       let bytes = read_to_bytes(dir, name)?; 
       let img = DynamicImage::from(load_image!(bytes.as_slice()).to_rgba8());

       let dimensions = img.dimensions();
       let image = Image {
            image: img,
            size: dimensions 
       };

       Ok(Self::Rgba(image))
    }

    pub fn new_luma(dir: &str, name: &str) -> Result<Self, Error> {
 
       let bytes = read_to_bytes(dir, name)?; 
       let img = DynamicImage::from(load_image!(bytes.as_slice()).to_luma8());
       let dimensions = img.dimensions(); 
       let image = Image {
            image: img,
            size: dimensions 
       };

       Ok(Self::Luma(image))       
    }
}

#[macro_export]
macro_rules! load_image {
    ($a: expr) => {
        {
            use anyhow::bail;
            use crate::errors::utils::ImageError;
            match image::load_from_memory($a) {
                Ok(img) => img,
                Err(e) => bail!(ImageError::LoadImage(e))
            }
        }
    }
}

pub use load_image;

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
