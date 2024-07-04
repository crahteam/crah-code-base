use crate::consts::RESOURCES;
use crate::components::{
    Component,
    HtmlComponent  
};
use crate::attributes::AttributeSetter;
use crate::tags::*;

pub fn get_header() -> Component {
    
    let mut header = Component::new();

    header.add(
        header!(
            nav!(
                img!().src(&format!("{}{}", RESOURCES, "logo.png")).class("logo"),
                ul!(
                    li!(a!("Home").href("home")).class("interactive_text"),
                    li!(a!("Projects").href("projects")).class("interactive_text"),
                    li!(a!("Posts").href("posts")).class("interactive_text"),
                    li!(a!("Videos").href("videos")).class("interactive_text"),
                    li!(a!("Get Involved").href("get_involved")).class("interactive_text")
                )
            ).class("horizontal_nav")
        )        
    );

    header
}