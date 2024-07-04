use std::path::Path;
use anyhow::{
    Error,
    bail
};
use std::io::{
    BufReader,
    Cursor
};

use crate::errors::utils::ReadFile;
pub fn read_to_string(dir: &str, name: &str) -> Result<String, Error> {
	
	let path = Path::new(dir).join(name);
	
	let source: String = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => bail!(ReadFile::ReadToString(e))
    };
    
    Ok(source)
}

pub fn read_to_buffer(dir: &str, name: &str) -> Result<BufReader<Cursor<String>>, Error>{
	
	let source = read_to_string(dir, name)?;
	let cursor = Cursor::new(source);
	
    Ok(BufReader::new(cursor))
}

pub fn read_to_bytes(dir: &str, name: &str) -> Result<Vec<u8>, Error> {
    match std::fs::read(format!("{}{}", dir, name)) {
        Ok(b) => Ok(b),
        Err(e) => bail!(ReadFile::ReadToBytes(e))
    }
}
