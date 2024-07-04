use image::ImageError as WImageError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ImageError{
    #[error("The image coudln't load from the bytes provided. {0}")]
    LoadImage(WImageError),
    #[error("The image couldn't be opened by the given file name. {0}")]
    OpenImageError(WImageError),
    #[error("Failed to get the image dimensions. {0}")]
    GetSizeError(WImageError)
}

#[derive(Error, Debug)]
pub enum ReadFile {
    #[error("Couldn't load the file to a string. {0}")]
    ReadToString(std::io::Error),
    #[error("Couldn't load the file to bytes. {0}")]
    ReadToBytes(std::io::Error)
}
