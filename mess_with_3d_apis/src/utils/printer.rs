// these macros printout POSITIVE or NEGATIVE strings. Using the CROSSTERM crate.
// print positive values 
#[macro_export]
macro_rules! ppos {
    () => {
        {
        // import
        use std::io::stdout;
        use crossterm::execute;
        use crossterm::style::{
            SetAttribute,
            SetForegroundColor,
            ResetColor,
            Color,
            Attribute
        };
        // set green + set bold
        execute!(stdout(), SetAttribute(Attribute::Bold), SetForegroundColor(Color::Rgb{r:0, g: 255, b:0}));
        println!("Success!");
        // remove green + remove bold
        execute!(stdout(), SetAttribute(Attribute::NoBold), ResetColor);
        }};

    ($a: expr) => {
        {
        // import
        use std::io::stdout;
        use crossterm::execute;
        use crossterm::style::{
            SetAttribute,
            SetForegroundColor,
            ResetColor,
            Color,
            Attribute
        };
        // set green + set bold
        execute!(stdout(), SetAttribute(Attribute::Bold), SetForegroundColor(Color::Rgb{r: 0, g: 255, b: 0}));
        println!("{}", $a);
        // remove green + remove bold
        execute!(stdout(), SetAttribute(Attribute::NoBold), ResetColor);
        }
    }
}

// prints negative values
#[macro_export]
macro_rules! pneg {
    ($a: expr) => {
        {
        // import
        use std::io::stdout;
        use crossterm::execute;
        use crossterm::style::{
            SetAttribute,
            SetForegroundColor,
            ResetColor,
            Color,
            Attribute,
        };
        // set red + set bold + set blinking text
        execute!(stdout(), SetAttribute(Attribute::Bold), SetAttribute(Attribute::SlowBlink), SetForegroundColor(Color::Rgb{r: 255, g: 0, b: 0}));
        println!("ERROR[{}]", $a);
        // remove red + remove bold + remove blinking text
        execute!(stdout(), SetAttribute(Attribute::NoBold), SetAttribute(Attribute::NoBlink), ResetColor);
        }
    } 
}

// match a result and print a success message if Ok() or the error if Err()
#[macro_export]
 macro_rules! pres{
    ($a: expr) => {
            match $a {
                Ok(s) => { ppos!()},
                Err(e) => { pneg!(e)}
            }
    }
}

// make the macros available everywhere
pub use pres;
pub use ppos;
pub use pneg;
