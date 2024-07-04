pub mod graphics;
pub mod physics;
pub mod maths;
pub mod utils;
pub mod audio;
pub mod errors;
pub mod user;

pub trait Update {
	fn update(&mut self);
}

