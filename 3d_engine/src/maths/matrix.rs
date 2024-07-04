use cgmath::{
	Deg,
	Angle
};

pub struct Matrix2([[f32; 2]; 2]);
pub struct Matrix3([[f32; 3]; 3]);
pub struct Matrix4([[f32; 4]; 4]);

impl Matrix4 {
	pub fn projection(fov: Deg::<f32>, aspect: f32, znear: f32, zfar: f32) -> Self {
		let mut m: Matrix4 = [[0;4];4];
		
		m[0][0] = aspect * (1 / (fov / 2).tan());
		m[1][1] = 1 / (fov / 2).tan();
		m[2][2] = zfar / (zfar - znear);
		m[2][3] = (-zfar * znear) / (zfar - znear);
		m[3][2] = 1.0;
		
		return m;
	}
}

pub fn rotate_vector(vec: &mut cgmath::Vector4::<f32>, a: Deg::<f32>) {
	
	let new_x = vec.x * a.cos() - vec.z * a.sin();
	let new_z = vec.x * a.cos() + vec.z * a.sin();
	
	vec.x = new_x;
	vec.z = new_z;
	 
}
