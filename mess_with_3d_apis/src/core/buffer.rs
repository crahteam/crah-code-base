use wgpu::{
    Device,
    util::BufferInitDescriptor,
    BufferUsages,
    VertexAttribute,
    VertexBufferLayout,
    VertexStepMode,
    Buffer,
    util::DeviceExt
};


pub fn create_buffer<E: bytemuck::Pod>(
    device: &Device,
    content: Vec<E>,
    buf_usage: BufferUsages) -> Buffer {

        let buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Buffer Creation"),
                contents: bytemuck::cast_slice(&content),
                usage: buf_usage 
            }
        );

        buffer
}

pub fn create_vertex_buffer_layout<'a>(
        size: usize,
        attributes: &'a [VertexAttribute],
        step_mode: VertexStepMode
    ) -> VertexBufferLayout<'a> {

    VertexBufferLayout {
        array_stride: size as u64,
        step_mode,
        attributes,
    }
}

// to create a vertex attribute blazingly fast
#[macro_export]
macro_rules! vertex_attr {
    ($a:expr, $b:expr, $c:expr) => {
        wgpu::VertexAttribute {
            format: $a,
            offset: $b as u64,
            shader_location: $c
        }
    }
}

pub use vertex_attr;
