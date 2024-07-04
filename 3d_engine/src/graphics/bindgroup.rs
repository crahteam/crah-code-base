use wgpu::{
    Device,
    Queue,
    BindGroup,
    BindGroupLayout,
    BindGroupDescriptor
};

pub enum BGLEntry {
    Texture,
    Sampler,
    Buffer
}

#[macro_export]
macro_rules! bind_group_layout_entry{
    //the binding, visibility, ty
    ($a:expr,$b:expr,$c:expr) => {
        {
        let mut ty: wgpu::BindingType;
        // depending on the BGLEntry, we have a different ty
        match $c {
            BGLEntry::Texture => {
                ty = wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {filterable: true}
                };
            },
            BGLEntry::Sampler => {
                ty = wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
            },
            BGLEntry::Buffer => {
                ty = wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                }
            }
        }

        // we build up the bind group layout entry
        wgpu::BindGroupLayoutEntry {
            binding: $a,
            visibility: $b,
            ty: ty,
            count: None
        }
      }
    }
}

#[macro_export]
macro_rules! bind_group_entry{
    ($a:expr, $b:expr) => {
        wgpu::BindGroupEntry {
            binding: $a,
            resource: $b
        }
    }
}

pub use bind_group_layout_entry;
pub use bind_group_entry;

pub fn create_bind_group(
    device: &Device,
    bgl: &BindGroupLayout,
    entries: &[wgpu::BindGroupEntry],
    label: Option<&str>
    ) -> BindGroup{
    
    device.create_bind_group(
        &BindGroupDescriptor {
            layout: bgl,
            entries,
            label
        }
    )
}

pub fn create_bind_group_layout(
    device: &Device,
    entries: &[wgpu::BindGroupLayoutEntry],
    label: Option<&str>
    ) -> BindGroupLayout {
    
    device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            entries,
            label
        }
    )
}
