// OPEN TAGS
#[macro_export]
macro_rules! open_tag {
    {$b: expr, $($a: expr), *} => {
        {
            let mut string = format!("<{}>", $b);
            $(string = format!("{}{}", string, $a);)*;
            string = format!("{}</{}>", string, $b);
            string
       }
    }
}

#[macro_export]
macro_rules! p{{$($a: expr), *} => 
    {{ use crate::tags::open_tag; open_tag!("p", $($a), *) }}}

#[macro_export]
macro_rules! main{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("main", $($a), *) }}}

#[macro_export]
macro_rules! li{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("li", $($a), *) }}}

#[macro_export]
macro_rules! h1{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h1", $($a), *) }}}

#[macro_export]
macro_rules! h2{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h2", $($a), *) }}}

#[macro_export]
macro_rules! h3{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h3", $($a), *) }}}

#[macro_export]
macro_rules! h4{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h4", $($a), *) }}}

#[macro_export]
macro_rules! h5{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h5", $($a), *) }}}

#[macro_export]
macro_rules! h6{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("h6", $($a), *) }}}

#[macro_export]
macro_rules! ul{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("ul", $($a), *) }}}

#[macro_export]
macro_rules! header{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("header", $($a), *) }}}

#[macro_export]
macro_rules! footer{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("footer", $($a), *) }}}

#[macro_export]
macro_rules! form{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("form", $($a), *) }}}

#[macro_export]
macro_rules! body{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("body", $($a), *) }}}

#[macro_export]
macro_rules! div{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("div", $($a), *) }}}

#[macro_export]
macro_rules! head{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("head", $($a), *) }}}

#[macro_export]
macro_rules! html{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("html", $($a), *) }}}

#[macro_export]
macro_rules! script{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("script", $($a), *) }}}

#[macro_export]
macro_rules! button{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("button", $($a), *) }}}

#[macro_export]
macro_rules! a{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("a", $($a), *) }}}

#[macro_export]
macro_rules! audio{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("audio", $($a), *) }}}

#[macro_export]
macro_rules! i{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("i", $($a), *) }}}

#[macro_export]
macro_rules! b{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("b", $($a), *) }}}

#[macro_export]
macro_rules! nav{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("nav", $($a), *) }}}

#[macro_export]
macro_rules! video{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("video", $($a), *) }}}

#[macro_export]
macro_rules! article{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("article", $($a), *) }}}

#[macro_export]
macro_rules! label{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("label", $($a), *) }}}

#[macro_export]
macro_rules! canvas{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("canvas", $($a), *) }}}

#[macro_export]
macro_rules! title{{$($a: expr), *} =>
     {{ use crate::tags::open_tag; open_tag!("title", $($a), *) }}}

pub use open_tag;

pub use p;
pub use main;
pub use li;
pub use h1;
pub use h2;
pub use h3;
pub use h4;
pub use h5;
pub use h6;
pub use ul;
pub use header;
pub use footer;
pub use form;
pub use body;
pub use html;
pub use div;
pub use head;
pub use script;
pub use label;
pub use article;
pub use a;
pub use audio;
pub use nav;
pub use i;
pub use video;
pub use canvas;
pub use title;

// CLOSED TAGS
#[macro_export]
macro_rules! closed_tag {
    {$b: expr} => {
        {
            let mut string = format!("<{}>", $b);
            string
       }
    }
}

#[macro_export]
macro_rules! input{() =>
    {{ use crate::tags::closed_tag; closed_tag!("input")}}}

#[macro_export]
macro_rules! meta{() =>
    {{ use crate::tags::closed_tag; closed_tag!("meta")}}}

#[macro_export]
macro_rules! doctype{() =>
    {{ use crate::tags::closed_tag; closed_tag!("!DOCTYPE html")}}}

#[macro_export]
macro_rules! br{() =>
    {{ use crate::tags::closed_tag; closed_tag!("br")}}}

#[macro_export]
macro_rules! source{() =>
    {{ use crate::tags::closed_tag; closed_tag!("source")}}}

#[macro_export]
macro_rules! img{() =>
    {{ use crate::tags::closed_tag; closed_tag!("img")}}}

pub use closed_tag;

pub use input;
pub use meta;
pub use doctype;
pub use br;
pub use source;
pub use img;