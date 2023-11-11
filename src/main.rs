mod furry_mesh;
mod camera;
mod state;

use egui_macroquad::{egui, macroquad};
use camera::OrbitCamera;
use furry_mesh::FurryMesh;
use macroquad::prelude::*;

#[macroquad::main("shell")]
async fn main() {

	let material = load_material(
		ShaderSource::Glsl { vertex: include_str!("vertex.glsl"), fragment: include_str!("fragment.glsl") }, 
		MaterialParams { 
			pipeline_params: PipelineParams { 
				cull_face: miniquad::CullFace::Nothing, 
				depth_test: Comparison::Less, 
				depth_write: true, 
				..Default::default() 
			}, 
			uniforms: vec![("CameraPos".to_owned(), UniformType::Float3)], 
			textures: vec![],
		}).unwrap();

	let mut orbit_camera = OrbitCamera {
		polar: 2.0,
		azimuth: 1.0,
		..Default::default()
	};

	let mut camera;

	let monkey = FurryMesh::from_file("./objs/monkey.obj");

	loop {
		clear_background(BLACK);
		
		orbit_camera.update(true);
		camera = orbit_camera.camera();
		material.set_uniform("CameraPos", camera.position);

		set_camera(&camera);

		draw_line_3d(Vec3::ZERO, Vec3::X * 10.0, RED);
		draw_line_3d(Vec3::ZERO, Vec3::Y * 10.0, GREEN);
		draw_line_3d(Vec3::ZERO, Vec3::Z * 10.0, BLUE);

		gl_use_material(&material);
		monkey.draw();

		next_frame().await;
	}
}
