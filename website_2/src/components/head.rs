use crate::consts::{
    RESOURCES,
    STYLES
};
use crate::components::{
    Component,
    HtmlComponent
};
use crate::attributes::AttributeSetter;
use crate::tags::*;
// keywords, description, styles
pub fn get_head(k: Option<&str>, d: Option<&str>, s: Vec<&str>) -> Component {

    let mut head = Component::new();
     
    let mut k = match k {
        None => {
            "crah, flat, scooter, community, henke palm"
        },
        Some(v) => v
    };
             
    let mut d = match d {
        None => {
            "Crah is a community of committed flat scooter rider, fighting for their dreams."
        },
        Some(s) => s
    };

    let mut styles = String::new();
    
    for style in s {
        styles = format!("
            {}<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\"/>", styles, format!("{}{}", STYLES, style)
        );    
    }
    
    head.add(
        meta!().charset("UTF-8"),
    );
    head.add(
        meta!().name("viewport").cont("width=device-width"),
    );
    head.add(
        meta!().name("author").cont("Tommaso Sana"),
    );
    head.add(
        meta!().name("description").cont(d),
    );
    head.add(
        meta!().name("keywords").cont(k),
    );
    head.add(
        styles
    );

    head
}