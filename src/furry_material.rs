use egui_macroquad::macroquad::prelude::*;

use crate::{spring::Spring, params::Params};

pub struct FurryMaterial {
	material: Material,
	pub spring: Spring,
}

impl FurryMaterial {
	pub fn new(params: &Params) -> Self {
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
					("WindPower".to_owned(), UniformType::Float1), 
					("WindTurbulence".to_owned(), UniformType::Float1), 
					("WindSpeed".to_owned(), UniformType::Float1),
					("Time".to_owned(), UniformType::Float1), 
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
					("SkinCol".to_owned(), UniformType::Float3),
					("BaseCol".to_owned(), UniformType::Float3),
					("TopCol".to_owned(), UniformType::Float3),
					("Ambient".to_owned(), UniformType::Float3),
					("Shading".to_owned(), UniformType::Float1),
				], 
				textures: vec![],
			}).unwrap();

		let ret = Self { 
			material,
			spring: Spring::new(),
		};

		ret.update_all(params);

		ret
	}

	pub fn activate(&self, params: &Params) {
		self.update_all(params);
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

	pub fn set_time(&self, time: f32) {
		self.material.set_uniform("Time", time);
	}

	pub fn update_all(&self, params: &Params) {
		self.material.set_uniform("WindPower", params.wind_power);
		self.material.set_uniform("WindSpeed", params.wind_speed);
		self.material.set_uniform("WindTurbulence", params.wind_turbulence);
		self.material.set_uniform("NumShells", params.shells);
		self.material.set_uniform("Length", params.length);
		self.material.set_uniform("LengthVar", params.length_var);
		self.material.set_uniform("Jitter", params.jitter);
		self.material.set_uniform("Thickness", params.thickness);
		self.material.set_uniform("Profile", params.profile);
		self.material.set_uniform("Density", params.density);
		self.material.set_uniform("SkinCol", params.skin_color);
		self.material.set_uniform("BaseCol", params.fur_color_base);
		self.material.set_uniform("TopCol", params.fur_color_top);
		self.material.set_uniform("Ambient", params.ambient);
		self.material.set_uniform("Shading", params.shading);
	}
}