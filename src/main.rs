mod furry_mesh;
mod camera;

use camera::OrbitCamera;
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
		center: vec3(0.0, 0.0, 0.0),
		polar: 2.0,
		azimuth: 1.0,
		zoom: -20.0,
		rotate_sinsitivity: 6.0,
		last_mouse: Vec2::from(mouse_position()) / vec2(screen_width(), screen_width()),
	};
	let mut camera;


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
		draw_sphere_ex(Vec3::ZERO, 1.0, None, WHITE, DrawSphereParams { rings: 100, slices: 100, draw_mode: DrawMode::Triangles });

		next_frame().await;
	}
}
