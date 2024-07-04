use wasm_bindgen::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub description: String
}

#[wasm_bindgen]
pub fn renderVideos() {
    
}

#[wasm_bindgen] 
pub fn renderVideo(id: u32) {
    
}