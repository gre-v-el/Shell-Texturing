use egui_macroquad::macroquad::prelude::*;

use crate::{camera::OrbitCamera, furry_mesh::FurryMesh};

pub struct State {
	orbit_camera: OrbitCamera,
	camera: Camera3D,
	mesh: FurryMesh,
}

impl State {
	pub fn new() -> Self {

		let orbit_camera = OrbitCamera {
			polar: 2.0,
			azimuth: 1.0,
			..Default::default()
		};

		let mesh = FurryMesh::from_file("./objs/monkey.obj");

		Self { orbit_camera, camera: Camera3D::default(), mesh }

	}

	pub fn draw(&mut self) {
		
		self.orbit_camera.update(true);
		self.camera = self.orbit_camera.camera();

		self.mesh.material.set_uniform("CameraPos", self.camera.position);
		set_camera(&self.camera);
		
		draw_line_3d(Vec3::ZERO, Vec3::X * 10.0, RED);
		draw_line_3d(Vec3::ZERO, Vec3::Y * 10.0, GREEN);
		draw_line_3d(Vec3::ZERO, Vec3::Z * 10.0, BLUE);
		

		self.mesh.draw();
	}

}