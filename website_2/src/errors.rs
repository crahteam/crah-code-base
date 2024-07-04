use thiserror::Error;

//#[derive(Error, Debug)]
//pub enum WriteToFile {
//    #[error("Couldn't load the file to a string. {0}")]
//    Read(std::io::Error),
//    #[error("Couldn't load the file to bytes. {0}")]
//    ReadToBytes(std::io::Error)
//}

#[derive(Error, Debug)]
#[error("File writing failed. {0}")]
pub struct WriteFileError(pub std::io::Error);