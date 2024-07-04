use anyhow::{
    Result,
    Error
};
use std::ops::Range;
use cgmath::Point3;
use crate::{
    errors::*,
    core::{
        image,
        image::{
            image_from_bytes
        },
        texture::{
            TextureData
        },
        buffer,
        buffer::{
            vertex_attr
        },
        texture,
        bindgroup,
        bindgroup::{
          bg_entry,
          bgl_entry,
        },
        vertex::{
            CoordsRange,
            CoordsBool,
            ModelVertex
        },
        camera::{
            Camera
        },
        collition::{
			CollitionBox
		}
    }

};

use log::debug;

use wgpu::{
    VertexBufferLayout,
    VertexFormat,
    Buffer,
    Device,
    Queue,
    BindGroupLayout,
    BindingResource,
    BufferUsages,
    VertexStepMode
};
use std::io::{BufReader, Cursor};

pub struct Model<'a> {
    pub meshes: Vec<Mesh<'a>>,
    pub materials: Vec<Material>,
    // ! the model's collitionbox is different and has a different centroid from
    //   its meshes.
    // ! you might prefer to detect collitions by iterating over each mesh's collition
    //   boxes. Using the model's collition box performs better, b
    pub collition_box: CollitionBox<'a>,
}

pub struct Mesh<'a> {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_elements: u32,
    pub material: usize,
    pub raw_vertices: Vec<ModelVertex>,
    // ! each mesh has a collition box with its own centroid
    pub collition_box: CollitionBox<'a>,
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub texture_data: TextureData
}

impl Model<'_> {
    pub async fn load_obj_model<'a>(
        device: & Device,
        queue: & Queue,
        texture_bgl: & BindGroupLayout,
        name: &'a str) -> Result<Model<'a>, Error> {

       let pat = std::path::Path::new("/home/spyro/Desktop/wtuxFPS/src").join(name);
       let obj_source_string: String = match std::fs::read_to_string(pat) {
           Ok(s) => s,
           Err(e) => bail!(LoadFile::LoadAsString(e))
       };

		   let obj_cursor = Cursor::new(obj_source_string);

		   let mut obj_reader = BufReader::new(obj_cursor);

		   
			let (models, obj_materials) = tobj::load_obj_buf_async(
				&mut obj_reader,
				&tobj::LoadOptions {
					triangulate: true,
					single_index: true,
					..Default::default()
				},
				|p| async move {
					//let mat_text = match std::fs::read_to_string(name) {
					//   Ok(s) => s,
					//   Err(e) => bail!(LoadFile::LoadAsString(e))
					//};
					//
					
					let path = std::path::Path::new("/home/spyro/Desktop/wtuxFPS/src")
						.join(p);
					let mat_text = std::fs::read_to_string(path).unwrap();
					tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
				},
			)
			.await?;

			let mut materials = Vec::new();
				for m in obj_materials? {
					println!("one material");
					// texture, view, sampler, bindgroup con bgl input -> TEXTUREDATA
				   // il nome è m.diffuse_texture
					println!("{}", m.diffuse_texture);
					println!("{:?}", m.diffuse); 
					let mut image;

					// if there's a name for the texture file (eg. "mytexture.png") load the file up
					if m.diffuse_texture != "" {
						let bytes = std::fs::read(&m.diffuse_texture[..])?;
						image = image_from_bytes!(&bytes);
					} else {
						// if the texture's file name is "", then we don't have a texture,
						// but we'll probably have a plain rgb color to use stored in m.diffuse

						image = image::ImageData::load_plain_rgb(m.diffuse.clone()).unwrap();
					}
					let texture = TextureData::create_texture(&device, &queue, image, Some("material texture"));
					let view = TextureData::create_view(&texture);
					let sampler = TextureData::create_sampler(&device);
				   
				   
					let texture_bg = bindgroup::create_bind_group(
						&device,
						texture_bgl,
						&[
							bg_entry!(0, BindingResource::TextureView(&view)),
							bg_entry!(1, BindingResource::Sampler(&sampler))
						],
						Some(" Texture bindgroup in obj loading")
					); 

					let texture_data: TextureData = TextureData {
						texture,
						texture_view: view,
						sampler: sampler,
						bind_group: Some(texture_bg),
						bind_group_layout: None
					};

					// creare texture bg, bgl e mettere in un TEXTURE DATA
					materials.push(Material {
						name: m.name,
						texture_data,
					})
			}
		
			 println!("{:?}", materials);
			
			 let mut meshes = models
					.into_iter()
					.map(|m| {
						let vertices = (0..m.mesh.positions.len() / 3)
							.map(|i| ModelVertex {
								position: [
									m.mesh.positions[i * 3],
									m.mesh.positions[i * 3 + 1],
									m.mesh.positions[i * 3 + 2],
								],
								tex_coords: [m.mesh.positions[i *3], m.mesh.positions[i * 3 +1]],
								normal: [
									m.mesh.normals[i * 3],
									m.mesh.normals[i * 3 + 1],
									m.mesh.normals[i * 3 + 2],
								],
							})
							.collect::<Vec<_>>();

			 let vertex_buffer = buffer::create_buffer(
				 &device,
				 vertices.clone(),
				 BufferUsages::VERTEX
			 );
		
			let indices_len = m.mesh.indices.len() as u32;
			let index_buffer = buffer::create_buffer(
				&device,
				m.mesh.indices,
				BufferUsages::INDEX
			);
			
			let collition_box = CollitionBox::new_from_vertices(name);
			
			Mesh {
					name: name.to_string(),
					vertex_buffer,
					index_buffer,
					num_elements: indices_len,
					material: m.mesh.material_id.unwrap_or(0),
					raw_vertices: vertices,
					collition_box,
				}
			})
			.collect::<Vec<_>>();
		for mut mesh in &mut meshes {
			CollitionBox::update_from_vertices(&mut mesh);
			
		}
		let collition_box = CollitionBox::new_from_meshes("mesh", &meshes);
		
		Ok(Model {
			meshes,
			materials,
			collition_box
		})
	   }
	}

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
            log::warn!("materials: {}", model.materials.len());
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group, camera, light_bind_group);
        }
    }
}
