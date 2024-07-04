// the string is supposed to be a tag, with nested stuff if you want to
pub trait AttributeSetter {
    fn add(&mut self, tag: &str, content: &str);
    fn id(self, s: &str) -> Self;
    fn class(self, s: &str) -> Self;
    fn dir(self, s: &str) -> Self;
    fn onclick(self, s: &str) -> Self;
    fn src(self, s:&str) -> Self;
    fn ty(self, s: &str) -> Self;
    fn name(self, s:&str) -> Self;
    fn cont(self, s: &str) -> Self;
    fn charset(self, s: &str) -> Self;
    fn href(self, s: &str) -> Self;
}

impl AttributeSetter for String {

    fn add(&mut self, tag: &str, content: &str) {
        let pieces = self.splitn(2, ">").collect::<Vec<&str>>();
        let mut new_string = String::new();
        let s = &format!(" {}=\"{}\">", tag, content)[..];
        new_string.push_str(pieces[0]);
        new_string.push_str(s);
        new_string.push_str(pieces[1]);
        *self = new_string;
    }

    // ATTRIBUTE SETTERS
    
    fn id(mut self, s: &str) -> Self {
        self.add("id", s);
        self
    }

    fn class(mut self, s: &str) -> Self {
        self.add("class", s);
        self
    }

    fn dir(mut self, s: &str) -> Self {
        self.add("dir", s);
        self
    }

    fn onclick(mut self, s: &str) -> Self {
        self.add("onclick", s);
        self
    }

    fn src(mut self, s: &str) -> Self {
        self.add("src", s);
        self
    }

    fn ty(mut self, s:&str) -> Self {
        self.add("type", s);
        self
    }

    fn name(mut self, s: &str) -> Self {
        self.add("name", s);
        self
    }

    fn cont(mut self, s: &str) -> Self {
        self.add("content", s);
        self
    }

    fn charset(mut self, s: &str) -> Self {
        self.add("charset", s);
        self
    }

    fn href(mut self, s: &str) -> Self {
        self.add("href", s);
        self
    }
}