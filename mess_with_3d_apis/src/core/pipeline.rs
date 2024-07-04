use wgpu::{
    PipelineLayout,
    Device,
    PipelineLayoutDescriptor,
    BindGroupLayout,
    SurfaceConfiguration,
    VertexBufferLayout,
    RenderPipeline,
    PrimitiveTopology,
    FrontFace,
    PolygonMode,
    PrimitiveState,
    ShaderModule

};

use crate::core::{
    texture::TextureData
};
pub fn create_pipeline_layout(device: &Device, bgls: &[&BindGroupLayout]) -> PipelineLayout {
    let pl = device.create_pipeline_layout(
        &PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: bgls,
            push_constant_ranges: &[]
        }
    );
    pl
}

pub fn create_pipeline(
    device: &Device,
    pipl: &PipelineLayout,
    shader: &ShaderModule,
    entry: (&str, &str),
    s_conf: &SurfaceConfiguration,
    vertex_buf_layout: &[VertexBufferLayout],
    depth_format: Option<wgpu::TextureFormat>,
    polygon_mode: PolygonMode,
    label: Option<&str>
    ) -> RenderPipeline {

    let p = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: label,
            layout: Some(pipl),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: entry.0,
                buffers: vertex_buf_layout 
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: entry.1,
                targets:  &[Some(wgpu::ColorTargetState {
                    format: s_conf.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: polygon_mode,
                conservative: false,
                
                //..Default::default()
            },
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
				format,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			}),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },  
            multiview: None
        }
    );
    p
}

