pub mod tags;
pub mod attributes;
pub mod consts;
pub mod pages;
pub mod components;
pub mod requests;
pub mod prelude;
pub mod errors;
pub mod router;

use crate::{
    consts::{
        ADDRESS  
    },
    tags::*,
    attributes::{
        AttributeSetter
    },
    pages::{
        HtmlPage,
        Page
    },
    components::{
        HtmlComponent,
        Component,
        header::get_header,
        footer::get_footer,
        head::get_head
    }
};

use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlImageElement,
    HtmlIFrameElement,
    MouseEvent
};
use serde::{
    Serialize,
    Deserialize
};

pub fn run() {
  
    let mut header = get_header();
    let mut footer = get_footer();
    let mut head = get_head(None, None, vec!["style.css"]);
    let mut index = Page::new("index.html");

    index.add(
        doctype!()
    );

    index.add(
        head.gen()  
    );    

    index.add(
        body!(
            header.gen(),
            main!(),
            footer.gen()
        )
    );

    index.add(
        script!().ty("module").src("./import_wasm.js")
    );
    
    index.gen_file();

}

