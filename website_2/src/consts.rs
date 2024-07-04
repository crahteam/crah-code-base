pub const ADDRESS: &str = "www.crah.app";
pub const RESOURCES: &str = "./res/";
pub const STYLES: &str = "./";
pub fn const_matcher(s: &str) -> &str {
    match s {
        "ADDRESS" => ADDRESS,
        _ => ""
    }
}