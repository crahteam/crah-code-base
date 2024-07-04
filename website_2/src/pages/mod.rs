
pub mod videos;
pub mod home;
pub mod posts;
pub mod projects;
pub mod get_involved;
pub mod error;

use anyhow::{
    Result,
    Error,
    bail
};
use crate::create_file;

pub trait HtmlPage {
    fn new(name: &str) -> Self;
    fn add(&mut self, s: String);
    fn gen_file(&self) -> Result<String>;
    fn gen(&self) -> String;
}

pub struct Page {
    pub name: String,
    pub content: Vec<String>
}

impl HtmlPage for Page {

    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            content: Vec::new()
        }
    }

    fn add(&mut self, s: String) {
        self.content.push(s);
    }

    fn gen_file(&self) -> Result<String> {
        let content = self.gen();
        create_file!(&self.name, &content.clone().into_bytes())?;
        Ok(content)
    }

    fn gen(&self) -> String {
        let mut content = String::new();
        for s in &self.content {
            content = format!("{}{}", content, s);
        }
        content
    }
}

#[macro_export]
macro_rules! create_file {
    ($a: expr, $b: expr) => {
        {
            use std::io::Write;
            use crate::errors;
            let mut file = std::fs::File::create($a);
            match file {
                Ok(mut f) => f.write($b),
                Err(e) => bail!(errors::WriteFileError(e))
            }           
        }
    }
}

