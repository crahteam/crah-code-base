use wasm_bindgen::prelude::*;
use crate::{
    consts::{
        RESOURCES
    }
};
use web_sys::{
    HtmlVideoElement
};
#[wasm_bindgen]
pub fn renderHome() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // BACKGROUND VIDEO
    
    let main = document.get_elements_by_tag_name("main").item(0).unwrap();
    let body = document.get_elements_by_tag_name("body").item(0).unwrap();
    let video_wrapper = document.create_element("div").unwrap();
    video_wrapper.set_attribute("class", "background-wrapper");
    let source = document.create_element("source").unwrap();
    source.set_attribute("src", &format!("{}{}", RESOURCES, "background_home.mp4"));
    source.set_attribute("type", "video/mp4");
    let video = document.create_element("video").unwrap();
    let video = video.dyn_into::<HtmlVideoElement>().unwrap();
    video.set_attribute("class", "background, video-background");
    video.set_autoplay(true);
    video.set_muted(true);
    video.set_loop(true);
    video.set_attribute("autobuffer", "true");
    video.set_attribute("playsinline", "true");
    video.append_child(&source);

    //let image = document.create_element("img").unwrap();
    //image.set_attribute("class", "background, image-background");
    //image.set_attribute("src", &format!("{}{}", RESOURCES, "background_image.png"));
    main.insert_adjacent_element("beforebegin", &video_wrapper);
    video_wrapper.append_child(&video);
    video.play();
    // TEXT PARAGRAPHS OVER THE VIDEO

    let introduction_box = document.create_element("div").unwrap();
    introduction_box.set_attribute("class", "window-box, translucent-dark-low");

    main.append_child(&introduction_box);
    
    let text_box_1 = document.create_element("div").unwrap();
    text_box_1.set_attribute("class", "text-box, text-box--landing-center");

    text_box_1.set_inner_html(
        "<h1 class=\"text-box__title\">We are Crah!</h1>
         <p class=\"text-box__content\">We are the flat scooter community.
            Crah is born with the goal of giving freedom and new chances
            to the ones that would like to ride flat until the end.
            I pushed for the first mile of a earth-to-moon journey.
            Now I need the help of every flat rider.
            We must make it to keep the magic going.</p>
        "
    );

    introduction_box.append_child(&text_box_1);
//    introduction_box.append_child(&text_box_1);
//    let we_are_crah = document.create_element("h1").unwrap();
//    we_are_crah.set_attribute("class", "text-box__title");
//    introduction_box.append_child(&we_are_crah);

    // TEXT PARAGRAPHS

    let text_box_2 = document.create_element("div").unwrap();
    text_box_2.set_attribute("class", "text-box");

    text_box_2.set_inner_html("
        <h1 class=\"text-box__title\">Scrap | The Crah's platform</h1>        
        <p class=\"text-box__content\">
        Crah will have a huge platform for content-sharing.
        Since I believe flat is much more than just a web page,
        I want to transmit you the whole vision I have.
        The platform is going to be accessible via a 3D scene,
        or a videogame if you want to.
        So you'll be able to post, read articles, watch flat scooter edits,
        chat with flat riders, inside the game.
        I'm working on an opensource game engine
        written in Rust, called Scrap.
        </p>
    ");
    
     main.append_child(&text_box_2); 

    let illustration_1 = document.create_element("img").unwrap();
    illustration_1.set_attribute("class", "illustration");
    illustration_1.set_attribute("src", &format!("{}{}", RESOURCES, "servercluster.svg"));
    main.append_child(&illustration_1);

    let text_box_3 = document.create_element("div").unwrap();
    text_box_3.set_attribute("class", "text-box");

    text_box_3.set_inner_html(
      " <h1 class=\"text-box__title\">Get Involved</h1>
        <p class=\"text-box__content\">Everything is going to be free in it's true meaning.
        Everyone can contribute with code, graphics, 3D models,
        articles, video shots, audios.
        But the most important part are the edits. We'll organize edits and documentaries with riders around all over
        the world. The best thing you can do to help is to go outside and film the craziest tricks you can,
        because that's what flat scootering is.
        And that's also what Crah is about: dirt, nature, freedom, happiness, teaming, tricks, edits,
        dreams; We provide tech-related resources just to let the platform grow.            
        </p>
        "  
    );

    main.append_child(&text_box_3);

    let illustration_2 = document.create_element("img").unwrap();
    illustration_2.set_attribute("class", "illustration");

    illustration_2.set_attribute("src", &format!("{}{}", RESOURCES, "opensource.svg"));
    main.append_child(&illustration_2);
}