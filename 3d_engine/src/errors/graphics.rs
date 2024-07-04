use thiserror::Error;
use winit::{
    error::{
        OsError,
    },
    window::BadIcon
};
use wgpu::{
	RequestDeviceError,
    CreateSurfaceError as WCreateSurfaceError,
    SurfaceError as WSurfaceError
};

#[derive(Error, Debug)]
#[error("No adapters were found.")]
pub struct AdapterError;

#[derive(Error, Debug)]
#[error("The device request failed. {0}")]
pub struct DeviceError(pub RequestDeviceError);

#[derive(Error, Debug)]
pub enum SurfaceError {
    #[error("The surface creation failed. {0}")]
    CreateSurfaceError(WCreateSurfaceError),
    #[error("The current texture couldn't be loaded from the surface. {0}")]
    GetCurrentTexture(WSurfaceError)
}

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("The creation of the window failed due to an OS error. {0}")]
    CreateWindowError(OsError),
}

#[derive(Error, Debug)]
pub enum IconError {
    #[error("The creation of the window's icon failed. {0}")]
    CreateIconError(BadIcon),
    #[error("The icon couldn't be loaded because image wasn't wrapped in a ImageReader::Rgba()")]
    NotRgba
}
