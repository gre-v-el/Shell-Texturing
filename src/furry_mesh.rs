use core::panic;
use std::path::Path;

use macroquad::{prelude::*, models::Vertex};
use obj::{Obj, IndexTuple, ObjData};

pub struct FurryMesh {
	mesh: Mesh,
}

impl FurryMesh {
	pub fn from_mesh(mesh: Mesh) -> Self {
		Self { 
			mesh
		}
	}

	pub fn from_file(path: impl AsRef<Path>) -> Self {
		let ObjData {
			normal,
			material_libs: _,
			objects,
			position,
			texture,
		} = Obj::load(path).unwrap().data;

		let mut vertices = Vec::new();
		let mut indices = Vec::new();

		for object in objects {
			for group in object.groups {
				for poly in group.polys {
					if poly.0.len() != 3 { panic!(); }

					for IndexTuple(pos, tex, norm) in poly.0 {
						let tex = 
							if let Some(tex) = tex { 
								texture[tex].into() 
							} 
							else { 
								vec2(0.0, 0.0) 
							};

						let norm = 
							if let Some(norm) = norm { 
								normal[norm]
							} 
							else { 
								[0.0, 0.0, 0.0]
							};

						let col = Color { r: norm[0], g: norm[1], b: norm[2], a: 1.0 };
						

						indices.push(vertices.len() as u16);
						vertices.push(Vertex {
							position: position[pos].into(),
							uv: tex,
							color: col,
						});
					}
				}
			}
		}

		let mesh = Mesh {
			texture: None,
			indices,
			vertices,
		};

		Self::from_mesh(mesh)
	}
}