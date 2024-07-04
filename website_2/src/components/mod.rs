pub trait HtmlComponent {
    fn new() -> Self;
    fn add(&mut self, s: String);
    fn gen(&self) -> String;
}

pub struct Component {
    pub content: Vec<String>
}

impl HtmlComponent for Component {

    fn new() -> Self {
        Self {
            content: Vec::new()
        }
    }

    fn add(&mut self, s: String) {
        self.content.push(s);
    }

    fn gen(&self) -> String {
        let mut content = String::new();
        for s in &self.content {
            content = format!("{}{}", content, s);
        }
        content
    }
}

pub mod header;
pub mod footer;
pub mod head;