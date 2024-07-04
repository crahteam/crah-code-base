use winit::{
    event_loop::EventLoop,
    dpi::{
        PhysicalSize
    },
    event::{
        VirtualKeyCode,
        Event
    }
};
use crate::audio::Audio;
use rodio::{
    OutputStreamHandle,
    OutputStream,
    Decoder
};

use std::collections::HashMap;

use crate::graphics::{
    window::WindowDescriptor,
    state::State,
    render,
    bindgroup::{
        bind_group_layout_entry,
        create_bind_group_layout,
        BGLEntry,
        bind_group_entry,
        create_bind_group
    },
    pipeline::{
        create_pipeline,
        create_pipeline_layout
    },
    texture::TextureData,
    game::{
        scene::Scene,
        model::{
            ModelDescriptor,
            Model
        },
        camera::{
            CameraDescriptor,
            CameraData,
            Camera
        },
        entity::{
            Entity
        },
        model::{
            Mesh,
            Material
        },
        controller::{
            Controller
        },
        vertex::Vertex,
        world::{
            World,
            terrain::{
                ProceduralTerrain,
                ProceduralTerrainDescriptor
            },
            sky::{
                Sky,
                SkyBackground
            }
        }
    }
};
use crate::user::Pipe;
use wgpu::{
    ShaderStages,
    include_wgsl,
    ShaderModuleDescriptor,
    RenderPipeline,
    BindGroup
};


pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group: &'a wgpu::BindGroup
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: std::ops::Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group: &'a wgpu::BindGroup
    );

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup, camera: &mut Camera, light_bind_group: &'a wgpu::BindGroup);

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: std::ops::Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group: &'a wgpu::BindGroup
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group: &'b wgpu::BindGroup
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group, camera, light_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        instances: std::ops::Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group: &'b wgpu::BindGroup
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.texture_data.bind_group.as_ref().unwrap(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup, camera: &mut Camera, light_bind_group: &'b wgpu::BindGroup) {
        self.draw_model_instanced(model, 0..1, camera_bind_group, camera, light_bind_group);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: std::ops::Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        camera: &mut Camera,
        light_bind_group:  &'b wgpu::BindGroup,

    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group, camera, light_bind_group);
        }
    }
}



use anyhow::Error;
pub fn render(state: &State, scene: &str) -> Result<(), Error> {
    let scene = state.scenes.get(scene).unwrap(); 
    let output = state.surface_data.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    {
        // CREATE RENDERPASS
        let mut sky: [f64; 3] = [ 0.0, 0.0, 0.0];
        if let SkyBackground::Rgba(array) = scene.world.sky.background {
            sky = array;
        }

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: sky[0],
                        g: sky[1],
                        b: sky[2],
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                 view: &state.depth_texture.view,
                 depth_ops: Some(wgpu::Operations {
                     load: wgpu::LoadOp::Clear(1.0),
                     store: true,
                 }),
                 stencil_ops: None,
         }),
        });

        //
        
        let mut last_pipe: &Pipe = &Pipe::None;

        for (_role, entity) in scene.entities.iter() {

            if &entity.pipeline != last_pipe {
                rpass.set_pipeline(scene.pipelines.get(&entity.pipeline ).unwrap());
                last_pipe = &entity.pipeline;
            }
           // println!("rendering an entity");

            // every pipeline has different things to do. by passing the scene the code has
            // complete control over the bindgroups and more.
            crate::user::pipeline_matcher!(&mut rpass, entity, scene);
            
        }
        
    }

    &state.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())

}


