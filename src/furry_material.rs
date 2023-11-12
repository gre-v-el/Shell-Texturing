use std::ops::RangeInclusive;

use egui_macroquad::{macroquad::prelude::*, egui::{Ui, DragValue, emath::Numeric}};

pub struct FurryMaterial {
	material: Material,
	shells: i32,
	length: f32,
	length_var: f32,
	jitter: f32,
	thickness: f32,
	profile: f32,
	density: f32,
}

impl FurryMaterial {
	pub fn new() -> Self {
		let material = load_material(
			ShaderSource::Glsl { vertex: include_str!("vertex.glsl"), fragment: include_str!("fragment.glsl") }, 
			MaterialParams { 
				pipeline_params: PipelineParams { 
					cull_face: miniquad::CullFace::Nothing, 
					depth_test: Comparison::Less, 
					depth_write: true, 
					..Default::default() 
				},
				uniforms: vec![
					("CameraPos".to_owned(), UniformType::Float3), 
					("SpringPos".to_owned(), UniformType::Float3), 
					("CurShell".to_owned(), UniformType::Int1),
					("NumShells".to_owned(), UniformType::Int1), 
					("Length".to_owned(), UniformType::Float1),
					("LengthVar".to_owned(), UniformType::Float1),
					("Jitter".to_owned(), UniformType::Float1),
					("Thickness".to_owned(), UniformType::Float1),
					("Profile".to_owned(), UniformType::Float1),
					("Density".to_owned(), UniformType::Float1),
				], 
				textures: vec![],
			}).unwrap();

		let ret = Self { 
			material,
			shells: 64,
			length: 0.3,
			length_var: 0.4,
			jitter: 1.0,
			thickness: 0.7,
			profile: 0.77,
			density: 400.0,
		};

		ret.update_all();

		ret
	}

	pub fn activate(&self) {
		self.update_all();
		gl_use_material(&self.material);
	}

	pub fn set_cur_shell(&self, i: i32) {
		self.material.set_uniform("CurShell", i);
	}

	pub fn set_camera_pos(&self, pos: Vec3) {
		self.material.set_uniform("CameraPos", pos);
	}

	pub fn set_spring_pos(&self, pos: Vec3) {
		self.material.set_uniform("SpringPos", pos);
	}

	pub fn update_all(&self) {
		self.material.set_uniform("NumShells", self.shells);
		self.material.set_uniform("Length", self.length);
		self.material.set_uniform("LengthVar", self.length_var);
		self.material.set_uniform("Jitter", self.jitter);
		self.material.set_uniform("Thickness", self.thickness);
		self.material.set_uniform("Profile", self.profile);
		self.material.set_uniform("Density", self.density);
	}

	pub fn ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.shells, 		1..=200, 0.5, 		"Number of shells:");
		Self::drag_n_update(ui, &mut self.length, 		0.0..=2.0, 0.02, 	"Fur length:");
		Self::drag_n_update(ui, &mut self.length_var, 	0.0..=1.0, 0.01, 	"Fur length variation:");
		Self::drag_n_update(ui, &mut self.jitter, 		0.0..=1.0, 0.01, 	"Strands jitter:");
		Self::drag_n_update(ui, &mut self.thickness, 	0.0..=1.0, 0.01, 	"Strands thickness:");
		Self::drag_n_update(ui, &mut self.profile, 		0.0..=1.0, 0.01, 	"Strand profile:");
		Self::drag_n_update(ui, &mut self.density, 		50.0..=1000.0, 1.0, "Fur density:");
	}

	pub fn drag_n_update<Num: Numeric>(ui: &mut Ui, val: &mut Num, range: RangeInclusive<Num>, speed: f64, label: &str) {
		ui.label(label);
		ui.add(DragValue::new(val).clamp_range(range).speed(speed));
	}

	pub fn get_shells(&self) -> i32 {
		self.shells
	}
}