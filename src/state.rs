use std::{fs::read_dir, io::Error, ffi::OsString, f32::consts::PI};

use egui_macroquad::{macroquad::prelude::*, egui::{Window, Align2, ComboBox, RichText, Color32, Label, Grid, Button}};

use crate::{camera::OrbitCamera, furry_mesh::FurryMesh, furry_material::FurryMaterial, params::Params};

/// Holds the entire application state
pub struct State {
	params: Params,
	orbit_camera: OrbitCamera,
	camera: Camera3D,
	mesh: Option<FurryMesh>,
	material: FurryMaterial,
	files: Result<Vec<OsString>, Error>,
	current_file: usize,
	info_open: bool,
}

impl State {
	pub fn new() -> Self {
		let orbit_camera = OrbitCamera {
			polar: PI*0.65,
			azimuth: -PI*0.75,
			..Default::default()
		};
		let files = scan_files();

		let params = Params::default();

		let mut ret = Self { orbit_camera, camera: Camera3D::default(), mesh: None, material: FurryMaterial::new(&params), files, current_file: 0, params, info_open: true };
		ret.load_mesh(0);

		ret
	}

	/// Draws the 2D base grid
	pub fn draw_grid() {
		let radius = 10.0;

		for i in (-radius as i32)..=(radius as i32) {
			draw_line_3d(Vec3::X * radius + Vec3::Y * i as f32, -Vec3::X * radius + Vec3::Y * i as f32, if i == 0 { RED } else { GRAY });
			draw_line_3d(Vec3::Y * radius + Vec3::X * i as f32, -Vec3::Y * radius + Vec3::X * i as f32, if i == 0 { BLUE } else { GRAY });
		}
	}

	/// Draws a single frame
	pub fn draw(&mut self) {
		clear_background(Color::new(self.params.ambient[0] * 1.5, self.params.ambient[1] * 1.5, self.params.ambient[2] * 1.5, 1.0));

		self.camera = self.orbit_camera.camera();
		
		set_camera(&self.camera);

		if self.params.show_grid {
			Self::draw_grid();
		}

		if let Some(mesh) = &mut self.mesh {
			self.material.set_camera_pos(self.camera.position);
			mesh.draw(&mut self.material, &self.params);
			
			let d_mouse = mouse_delta_position();
			if is_key_down(KeyCode::LeftShift) && is_mouse_button_down(MouseButton::Left) {
				let d_mouse = d_mouse * self.camera.position.distance(mesh.get_position());
				let displacement = vec3(0.0, d_mouse.x, d_mouse.y);
				let displacement = Mat3::from_rotation_y(self.orbit_camera.polar - PI/2.0).mul_vec3(displacement);
				let displacement = Mat3::from_rotation_z(self.orbit_camera.azimuth).mul_vec3(displacement);
				mesh.displace(displacement, &mut self.material.spring);
			}
		}

		let pointer_free = self.ui();

		self.orbit_camera.update(pointer_free && !is_key_down(KeyCode::LeftShift));
	}

	pub fn ui(&mut self) -> bool {
		let mut choice = self.current_file;
		let mut can_drag = false;
		let mut reset_mat = false;
		egui_macroquad::ui(|ctx| {
			Window::new("Inspector")
				.anchor(Align2::LEFT_TOP, [10.0; 2])
				.fixed_size([150.0, 300.0])
				.title_bar(false)
				.show(ctx, |ui| {
					if ui.selectable_value(&mut self.info_open, true, "Show info").clicked() {

					}

					if let Ok(files) = &self.files {
						ui.label("Choose model:");
						ComboBox::new("models", "")
							.selected_text(files[self.current_file].to_string_lossy())
							.show_ui(ui, |ui| {
								for (i, file) in files.iter().enumerate() {
									ui.selectable_value(&mut choice, i, file.to_string_lossy());
								}
							});
						
						ui.separator();
						self.params.ui(ui);
						ui.separator();
						if ui.button("reset settings").clicked() {
							reset_mat = true;
						}
					}
					else {
						ui.add(Label::new(RichText::new("Couldn't read ./objs/\n").color(Color32::RED)));
					}
				});
			
			Window::new("Shell TextFURring").open(&mut self.info_open).show(ctx, |ui| {
				ui.label("[You can view this window again by clicking the first button in the inspector on the left]");
				ui.separator();

				ui.label("Controls:");

				Grid::new("controls").striped(true).show(ui, |ui| {
					ui.label("Drag LMB");
					ui.label("Rotate");
					ui.end_row();

					ui.label("Drag RMB");
					ui.label("Pan around");
					ui.end_row();

					ui.label("LShift + Drag LMB");
					ui.label("Move object");
					ui.end_row();
				});

				ui.separator();

				ui.label("In the inspector on the left you can select an object to show. The objects listed come from ./objs/ directory, where you can paste your own models. This program accepts only triangulated meshes, so triangulate it beforehand in some software (for example Blender). Have fun with playing around with parameters! If you have any more questions hit me up on discord: ");
			});
				
			can_drag = !(ctx.is_using_pointer() || ctx.is_pointer_over_area());
		});
		
		egui_macroquad::draw();
		if reset_mat {
			self.params = Params::default();
		}
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