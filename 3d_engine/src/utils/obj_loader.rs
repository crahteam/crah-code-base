// THIS LOADER USES TOBJ
use wgpu::{
    Device,
    Queue
};
use crate::physics::body::{
    SmallPhysicalBody,
    PhysicalBody
};
use crate::graphics::{
   texture::{
        TextureData
   },
   buffer,
   bindgroup,
   bindgroup::{
        bind_group_entry,
        bind_group_layout_entry
   }
};
use crate::graphics::game::{
    vertex::Vertex,
    model::{
        ModelDescriptor,
        Model,
        Material,
        Mesh
    }
};
use wgpu::{
    BindingResource,
    BindGroupLayout,
    BufferUsages
};
use crate::utils::{
    image::{
        load_image,
        Image,
        ImageReader
    },
    reader::{
        read_to_buffer
    }
};

use anyhow::Error;
use crate::user::DIR;
pub async fn load<'a>(
    device: &Device,
    queue: &Queue,
    texture_bgl: &BindGroupLayout,
    desc: &ModelDescriptor<'_>
) -> Result<Model, Error> {
	
	let mut buffer = read_to_buffer(desc.dir, desc.name)?;
    println!("buffer obj loaded");	
	let (models, obj_materials) = tobj::load_obj_buf_async(
		&mut buffer,
		&tobj::LoadOptions {
			triangulate: true,
			single_index: true,
			..Default::default()
		},
		|p| async move {
			let mut material_buffer = read_to_buffer(desc.dir, &p).unwrap();
			tobj::load_mtl_buf(&mut material_buffer)
	}).await?;

	let mut materials = Vec::new();
	
    println!("bout to load the materials");
	for m in obj_materials? { 
		
		let mut image;
		if m.diffuse_texture != "" {
			//let bytes = std::fs::read(&m.diffuse_texture[..])?;
			//image = ImageReader::Rgba(load_image!(&bytes));
            println!("{}", &m.diffuse_texture[..]);
            image = ImageReader::new_rgba("", &m.diffuse_texture[..])?;
		} else {
            println!("{:?}", &m.diffuse);
			image = ImageReader::plain_rgba(m.diffuse.clone())?;
            println!("done");
		}
		
		let texture = TextureData::new_texture(&device, &queue, image, Some("material texture"));
		let view = TextureData::new_view(&texture);
		let sampler = TextureData::new_sampler(&device);
				      
		let texture_bindgroup = bindgroup::create_bind_group(
			&device,
			texture_bgl,
			&[
				bind_group_entry!(0, BindingResource::TextureView(&view)),
				bind_group_entry!(1, BindingResource::Sampler(&sampler))
			],
			Some(" Texture bindgroup in obj loading")
		); 

		let texture_data: TextureData = TextureData {
			texture,
			view,
			sampler,
			bind_group: Some(texture_bindgroup),
			bind_group_layout: None
		};

		materials.push(Material {
			name: m.name,
			texture_data,
		})
		
	}

	let mut meshes = models.into_iter().map(|m| {
		let vertices = (0..m.mesh.positions.len() / 3).map(|i| Vertex {
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
		}).collect::<Vec<_>>();

		let vertex_buffer = buffer::create_buffer(
			&device,
			vertices.clone(),
			BufferUsages::VERTEX
		);
		
        let physics = SmallPhysicalBody::new(&vertices, &m.mesh.indices);
		let indices_len = m.mesh.indices.len() as u32;
		
		let index_buffer = buffer::create_buffer(
			&device,
			m.mesh.indices,
			BufferUsages::INDEX
		);
			
		//let collition_box = CollitionBox::new_from_vertices(name);
			
        Mesh {
                name: desc.name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: indices_len,
                material: m.mesh.material_id.unwrap_or(0),
                vertices,
                physics,
        }
	}).collect::<Vec<_>>();
	
    let model_phys = PhysicalBody::new(desc.mass, desc.speed, &meshes);
		
	Ok(Model {
		meshes,
		materials,
        physics: model_phys
	})
}

