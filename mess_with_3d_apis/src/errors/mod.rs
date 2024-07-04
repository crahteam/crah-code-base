use thiserror::Error;

// To provide simple error identifiers, some of them are overlapping
// with other crates' errors' identifiers. Those get renamed with W<original identifier>

use wgpu::{
    RequestDeviceError,
    CreateSurfaceError as WCreateSurfaceError,
    SurfaceError as WSurfaceError
};
use winit::{
    error::{
        OsError,
    },
    window::BadIcon
};

use image::ImageError as WImageError;

#[derive(Error, Debug)]
#[error("No adapters were found for the current instance.")]
pub struct AdapterError();

#[derive(Error, Debug)]
#[error("The device request on the adapter failed. {0}")]
pub struct DeviceQueueError(pub RequestDeviceError);

#[derive(Error, Debug)]
pub enum SurfaceError {
    #[error("The surface creation on the current instance failed. {0}")]
    CreateSurfaceError(WCreateSurfaceError),
    #[error("Couldn't load the current texture from the surface while rendering. {0}")]
    GetCurrentTexture(WSurfaceError)
}

#[derive(Error, Debug)]
pub enum ImageError{
    #[error("The image coudln't load from the bytes provided. {0}")]
    LoadFromBytesError(WImageError),
    #[error("The image couldn't be opened by the given file name. {0}")]
    OpenImageError(WImageError)
}

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("The creation of the window failed due to an OS error. {0}")]
    CreateWindowError(OsError),
    #[error("The creation of the window icon failed. {0}")]
    IconError(BadIcon)
}

#[derive(Error, Debug)]
pub enum LoadFile {
    #[error("Couldn't load the file as a string. {0}")]
    LoadAsString(std::io::Error)
}
