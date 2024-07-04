use crate::consts::RESOURCES;
use crate::components::{
    Component,
    HtmlComponent
};
use crate::attributes::AttributeSetter;
use crate::tags::*;

pub fn get_footer() -> Component {
    let mut footer = Component::new();

    footer.add(
        footer!(
            nav!(
                p!("2023 - We are Crah!")
            ).class("horizontal_nav")
        )  
    );

    footer
}