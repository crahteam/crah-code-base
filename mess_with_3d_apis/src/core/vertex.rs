use std::ops::Range;
use core::cmp::{min, max};
use crate::core::{
	buffer,
	buffer::{
		vertex_attr
	}
};

use wgpu::{
	VertexFormat,
	VertexStepMode,
	VertexBufferLayout
};

#[derive(Debug)]
pub struct CoordsRange {
    pub x: Range::<f32>,
    pub y: Range::<f32>,
    pub z: Range::<f32>
}

impl CoordsRange {
	pub fn overlap(range1: &Range::<f32>, range2: &Range::<f32>) -> bool {
		return (range1.start >= range2.start && range1.start <= range2.end) ||
				(range1.end >= range2.start && range1.end <= range2.end) ||
				(range2.start >= range1.start && range2.start <= range1.end) ||
				(range2.end >= range1.start && range2.end <= range1.end);
	}
	// returns true if there's an interesection in every coordinate of the 2 ranges
	pub fn compare_ranges(c1: &CoordsRange, c2: &CoordsRange) -> (bool, bool, bool) {
		
		let mut check: (bool, bool, bool) = (false, false, false);
		
		if Self::overlap(&c2.x, &c1.x) {
			check.0 = true;
		}
		if Self::overlap(&c2.y, &c1.y) {
			check.1 = true;
		}
		if Self::overlap(&c2.z, &c1.z) {
			check.2 = true;
		}
		
		check
	}
	
}

#[derive(Debug, PartialEq, Clone)]
pub struct CoordsBool {
    pub x: bool,
    pub y: bool,
    pub z: bool
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3]
}

impl <'a>ModelVertex {
    pub fn desc() -> VertexBufferLayout<'a> {
        let model_vertex_buf_layout = buffer::create_vertex_buffer_layout(
            std::mem::size_of::<ModelVertex>(),
            &[
                vertex_attr!(VertexFormat::Float32x3, 0, 0),
                vertex_attr!(VertexFormat::Float32x2, std::mem::size_of::<[f32; 3]>(), 1),
                vertex_attr!(VertexFormat::Float32x3, std::mem::size_of::<[f32; 5]>(), 2)
            ],
            VertexStepMode::Vertex
        );
        model_vertex_buf_layout
    }
}
