use core::panic;
use std::path::Path;

use egui_macroquad::macroquad;
use macroquad::{prelude::*, models::Vertex};
use obj::{Obj, IndexTuple, ObjData, Object};

const MAX_VERTS: usize = 10000;
const MAX_INDS: usize = 5000;

pub struct FurryMesh {
	pub meshes: Vec<Mesh>,
}

impl FurryMesh {

	pub fn empty() -> Self {
		Self { meshes: Vec::new() }
	}

	pub fn add_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u16>) {
		self.meshes.push(Mesh { vertices, indices, texture: None })
	}

	pub fn load_data(&mut self, objects: Vec<Object>, position: Vec<[f32; 3]>, texture: Vec<[f32; 2]>, normal: Vec<[f32; 3]>) {
		let mut vertices: Vec<Vertex> = Vec::new();
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
						let norm = 0.5 + 0.5 * Vec3::from(norm).normalize();

						let col = Color { r: norm.x, g: norm.y, b: norm.z, a: 1.0 };
						
						let mut index = None;
						for (i, vert) in vertices.iter().enumerate() {
							if vert.position == position[pos].into() && vert.uv == tex && vert.color == col {
								index = Some(i as u16);
							}
						}
						
						if let Some(i) = index {
							indices.push(i);
						}
						else {
							indices.push(vertices.len() as u16);
							vertices.push(Vertex {
								position: position[pos].into(),
								uv: tex,
								color: col,
							});
						}

					}

					if vertices.len() + 3 >= MAX_VERTS || indices.len() + 3 >= MAX_INDS {
						self.add_mesh(vertices, indices);
						vertices = Vec::new();
						indices = Vec::new();
					}
				}
			}
		}

		self.add_mesh(vertices, indices);
		
		let mut verts = 0;
		let mut inds = 0;
		for mesh in &self.meshes {
			verts += mesh.vertices.len();
			inds += mesh.indices.len();
		}

		println!("verts: {}", verts);
		println!("inds: {}", inds);
	}

	pub fn load_file(&mut self, path: impl AsRef<Path>) {
		
		let ObjData {
			normal,
			material_libs: _,
			objects,
			position,
			texture,
		} = Obj::load(path).unwrap().data;

		self.load_data(objects, position, texture, normal);
	}

	pub fn from_file(path: impl AsRef<Path>) -> Self {

		let mut furry_mesh = Self::empty();
		furry_mesh.load_file(path);

		furry_mesh
	}

	pub fn draw(&self) {
		for mesh in &self.meshes {
			draw_mesh(mesh);
		}
	}
}