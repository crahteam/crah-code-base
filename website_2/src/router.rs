use wasm_bindgen::prelude::*;

use crate::pages::{
    home::renderHome,
    projects::renderProjects,
    videos::{
        renderVideos,
        renderVideo
    },
    posts::{
        renderPosts,
        renderPost
    },
    get_involved::renderGetInvolved,
    error::{
        render404
    }
};

#[wasm_bindgen]
pub fn routerInit() {

    let window = web_sys::window().unwrap();
    let location = &window.location().pathname().unwrap();
    let mut v =  location.split('/').collect::<Vec<&str>>();
    v.remove(0);

    match v.as_slice() {
        [] => renderHome(),
        ["home"] => renderHome(),
        ["projects"] => renderProjects(),
        ["videos"] => renderVideos(),
        ["videos", id] => renderVideo(str_to_u32(id)),
        ["posts"] => renderPosts(), 
        ["posts", id] => renderPost(str_to_u32(id)),
        ["get_involved"] => renderGetInvolved(),
        _ => render404()
    }
}

#[wasm_bindgen]
pub fn str_to_u32(s: &str) -> u32 {
    let n: u32 = s.parse().unwrap();
    n
}