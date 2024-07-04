#[macro_export]
macro_rules! create_file {
    // a is the file name, b is the content in bytes
    ($a: expr, $b: expr) => {
        {
            let mut file = std::fs::File::create($a).expect("");
            file.write($b)?;
        } 
    }
}
pub use create_file;
