use anyhow::{
    bail,
    Error
};
use std::collections::HashMap;
use rodio::Decoder;
use crate::utils::reader::read_to_buffer;
use crate::errors::audio::AudioError;
use crate::user::AudioSource;
pub struct Audio {
   pub audio_handler: rodio::OutputStreamHandle,
   pub audios: HashMap<AudioSource, Decoder<std::io::BufReader<std::io::Cursor<String>>>> // rodio::Decoder
}

impl Audio {

    pub fn new_source(dir: &str, name: &str) -> Result<Decoder<std::io::BufReader<std::io::Cursor<String>>>, Error> {
        let file = read_to_buffer(dir, name)?;
        let source = match Decoder::new(file) {
            Ok(s) => s,
            Err(e) => bail!(AudioError::CreateDecoder(e))
        };

        Ok(source)
    }
}
