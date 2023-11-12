use std::{fs::read_dir, io::Error, ffi::OsString};

use egui_macroquad::{macroquad::prelude::*, egui::{Window, Align2, ComboBox, RichText, Color32, Label}};

use crate::{camera::OrbitCamera, furry_mesh::FurryMesh, furry_material::FurryMaterial};

pub struct State {
	orbit_camera: OrbitCamera,
	camera: Camera3D,
	mesh: Option<FurryMesh>,
	material: FurryMaterial,
	files: Result<Vec<OsString>, Error>,
	current_file: usize,
}

impl State {
	pub fn new() -> Self {
		let orbit_camera = OrbitCamera {
			polar: 2.0,
			azimuth: 1.0,
			..Default::default()
		};
		let files = scan_files();

		let mut ret = Self { orbit_camera, camera: Camera3D::default(), mesh: None, material: FurryMaterial::new(), files, current_file: 0, };
		ret.load_mesh(0);

		ret
	}

	pub fn draw(&mut self) {
		
		self.camera = self.orbit_camera.camera();
		
		set_camera(&self.camera);

		draw_line_3d(Vec3::ZERO, Vec3::X * 10.0, RED);
		draw_line_3d(Vec3::ZERO, Vec3::Y * 10.0, GREEN);
		draw_line_3d(Vec3::ZERO, Vec3::Z * 10.0, BLUE);

		if let Some(mesh) = &self.mesh {
			self.material.set_camera_pos(self.camera.position);
			mesh.draw(&self.material);
		}

		let can_drag = self.ui();

		self.orbit_camera.update(can_drag);
	}

	pub fn ui(&mut self) -> bool {
		let mut choice = self.current_file;
		let mut can_drag = false;
		egui_macroquad::ui(|ctx| {
			Window::new("Inspector")
				.anchor(Align2::LEFT_TOP, [10.0; 2])
				.fixed_size([100.0, 300.0])
				.title_bar(false)
				.show(ctx, |ui| {
					if let Ok(files) = &self.files {
						ui.label("Choose model:");
						ComboBox::new("models", "")
							.show_ui(ui, |ui| {
								for (i, file) in files.iter().enumerate() {
									ui.selectable_value(&mut choice, i, file.to_string_lossy());
								}
							});
						
						ui.separator();

						self.material.ui(ui);
					}
					else {
						ui.add(Label::new(RichText::new("Couldn't read ./objs/\n").color(Color32::RED)));
					}
				});


			can_drag = !(ctx.is_using_pointer() || ctx.is_pointer_over_area());
		});
		egui_macroquad::draw();
		if choice != self.current_file {
			self.load_mesh(choice);
		}

		can_drag
	}

	pub fn load_mesh(&mut self, i: usize) {
		if let Ok(files) = &self.files {
			self.current_file = i;
			if i >= files.len() {
				return;
			}
			let mut path = OsString::from("./objs/");
			path.push(&files[i]);
			self.mesh = FurryMesh::from_file(path);
		}
	}
}

fn scan_files() -> Result<Vec<OsString>, Error> {
	let mut ret = Vec::new();
	
	for file in read_dir("./objs/")? {
		ret.push(file?.file_name());
	}

	Ok(ret)
}