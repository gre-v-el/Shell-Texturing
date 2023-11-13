use std::ops::RangeInclusive;

use egui_macroquad::egui::{Ui, DragValue, emath::Numeric, color_picker::color_edit_button_rgb};

pub struct Params {
	pub wind_power: f32,
	pub wind_speed: f32,
	pub wind_turbulence: f32,
	pub shells: i32,
	pub length: f32,
	pub length_var: f32,
	pub jitter: f32,
	pub thickness: f32,
	pub profile: f32,
	pub density: f32,
	pub stiffness: f32,
	pub drag: f32,
	pub skin_color: [f32; 3],
	pub fur_color_base: [f32; 3],
	pub fur_color_top: [f32; 3],
	pub ambient: [f32; 3],
	pub shading: f32,
	pub show_grid: bool,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			shells: 64,
			length: 0.3,
			length_var: 0.4,
			jitter: 1.0,
			thickness: 0.7,
			profile: 0.77,
			density: 400.0,
			wind_power: 0.01,
			wind_speed: 0.3,
			wind_turbulence: 0.5, 
			stiffness: 40.0, 
			drag: 3.0,
			skin_color: [0.9, 0.7, 0.6],
			fur_color_base: [0.3, 0.2, 0.1],
			fur_color_top: [0.9, 0.7, 0.6],
			ambient: [0.15, 0.15, 0.15],
			shading: 0.45,
			show_grid: true,
		}
	}
}

impl Params {
	pub fn spring_ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.stiffness, 0.0..=1000.0, 5.0, "Stiffness");
		Self::drag_n_update(ui, &mut self.drag, 1.0..=20.0, 0.5, "Drag");
	}
	
	pub fn surface_ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.shells, 		1..=200, 0.5, 		"Number of shells");
		Self::drag_n_update(ui, &mut self.length, 		0.0..=2.0, 0.02, 	"Fur length");
		Self::drag_n_update(ui, &mut self.length_var, 	0.0..=1.0, 0.01, 	"Fur length variation");
		Self::drag_n_update(ui, &mut self.jitter, 		0.0..=1.0, 0.01, 	"Strands jitter");
		Self::drag_n_update(ui, &mut self.thickness, 	0.0..=1.0, 0.01, 	"Strands thickness");
		Self::drag_n_update(ui, &mut self.profile, 		0.0..=1.0, 0.01, 	"Strand profile");
		Self::drag_n_update(ui, &mut self.density, 		50.0..=1000.0, 1.0, "Fur density");
	}

	pub fn wind_ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.wind_speed,		0.01..=2.0, 0.005, "Wind speed");
		Self::drag_n_update(ui, &mut self.wind_turbulence,	0.01..=1.5, 0.01, "Wind turbulence");
		Self::drag_n_update(ui, &mut self.wind_power,		0.0..=0.05,  0.0005, "Wind power");
	}

	pub fn color_ui(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			color_edit_button_rgb(ui, &mut self.fur_color_top);
			ui.label("Fur color - top");
		});
		ui.horizontal(|ui| {
			color_edit_button_rgb(ui, &mut self.fur_color_base);
			ui.label("Fur color - base");
		});
		ui.horizontal(|ui| {
			color_edit_button_rgb(ui, &mut self.skin_color);
			ui.label("Skin color");
		});
	}

	pub fn shading_ui(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			color_edit_button_rgb(ui, &mut self.ambient);
			ui.label("Ambient light");
		});
		Self::drag_n_update(ui, &mut self.shading, 0.0..=1.0, 0.01, "Shading");
		ui.checkbox(&mut self.show_grid, "Show grid");
	}

	pub fn ui(&mut self, ui: &mut Ui) {
		ui.collapsing("Color", |ui| self.color_ui(ui));
		ui.collapsing("Fur",   |ui| self.surface_ui(ui));
		ui.collapsing("Physics", |ui| self.spring_ui(ui));
		ui.collapsing("Wind", |ui| self.wind_ui(ui));
		ui.collapsing("Rendering", |ui| self.shading_ui(ui));
	}
	
	pub fn drag_n_update<Num: Numeric>(ui: &mut Ui, val: &mut Num, range: RangeInclusive<Num>, speed: f64, label: &str) {
		ui.horizontal(|ui| {
			ui.add(DragValue::new(val).clamp_range(range).speed(speed));
			ui.label(label);
		});
	}
}