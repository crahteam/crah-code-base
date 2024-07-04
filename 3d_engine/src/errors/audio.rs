use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("An error occurred while decoding the source. {0}")]
    CreateDecoder(rodio::decoder::DecoderError),
}
