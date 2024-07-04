use wgpu::{
    Texture,
    TextureView,
    TextureViewDescriptor,
    TextureDescriptor,
    TextureDimension,
    TextureFormat,
	TextureUsages,
    TextureAspect,
	Sampler,
	BindGroup,
	BindGroupLayout,
	Device,
	Queue,
	Extent3d,
	ImageCopyTexture,
    ImageDataLayout,
    Origin3d,
    SamplerDescriptor
};

use crate::utils::image::ImageReader;
use image::GenericImageView;

#[derive(Debug)]
pub struct TextureData {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    // Options because we might want to not store it here, but reuse some others.
    pub bind_group: Option<BindGroup>,
    pub bind_group_layout: Option<BindGroupLayout>,
}

impl TextureData {

	pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
	
    pub fn new_texture(
        device: &Device,
        queue: &Queue,
        image_reader: ImageReader,
        label: Option<&str> 
    ) -> Texture {
			
		let mut dimensions;
        let mut bytes;

        if let ImageReader::Rgba(image) = image_reader {
            dimensions = image.size;
            bytes = image.image.to_rgba8();
         } else {
            panic!();   
        }

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };
        
        let texture = device.create_texture(
            &TextureDescriptor {
                size: size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                label: label,
                view_formats: &[]
            }
        );
        
        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            &bytes,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1)
            },
            size
        );

        texture
    }

	pub fn new_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
		
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[Self::DEPTH_FORMAT],
        };
        
        let texture = device.create_texture(&desc);
        
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::MirrorRepeat,
            address_mode_v: wgpu::AddressMode::MirrorRepeat,
            address_mode_w: wgpu::AddressMode::MirrorRepeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            bind_group: None,
            bind_group_layout: None
        }
    }
    
    pub fn new_view(texture: &Texture) -> TextureView {
		texture.create_view(
            &TextureViewDescriptor::default()
        )
    }

    pub fn new_sampler(device: &Device) -> Sampler {
        device.create_sampler(
            &SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::MirrorRepeat,
                address_mode_v: wgpu::AddressMode::MirrorRepeat,
                address_mode_w: wgpu::AddressMode::MirrorRepeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        )
    }
    
}
