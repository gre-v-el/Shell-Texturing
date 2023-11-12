use std::ops::RangeInclusive;

use egui_macroquad::egui::{Ui, DragValue, emath::Numeric};

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
			drag: 3.0
		}
	}
}

impl Params {
	pub fn spring_ui(&mut self, ui: &mut Ui) {
		ui.label("Stiffness:");
		ui.add(DragValue::new(&mut self.stiffness).clamp_range(0.0..=1000.0).speed(5.0));
		ui.label("Drag:");
		ui.add(DragValue::new(&mut self.drag).clamp_range(1.0..=20.0).speed(0.5));
	}
	
	pub fn surface_ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.shells, 		1..=200, 0.5, 		"Number of shells:");
		Self::drag_n_update(ui, &mut self.length, 		0.0..=2.0, 0.02, 	"Fur length:");
		Self::drag_n_update(ui, &mut self.length_var, 	0.0..=1.0, 0.01, 	"Fur length variation:");
		Self::drag_n_update(ui, &mut self.jitter, 		0.0..=1.0, 0.01, 	"Strands jitter:");
		Self::drag_n_update(ui, &mut self.thickness, 	0.0..=1.0, 0.01, 	"Strands thickness:");
		Self::drag_n_update(ui, &mut self.profile, 		0.0..=1.0, 0.01, 	"Strand profile:");
		Self::drag_n_update(ui, &mut self.density, 		50.0..=1000.0, 1.0, "Fur density:");
	}

	pub fn wind_ui(&mut self, ui: &mut Ui) {
		Self::drag_n_update(ui, &mut self.wind_speed,		0.01..=2.0, 0.005, "Wind speed:");
		Self::drag_n_update(ui, &mut self.wind_turbulence,	0.01..=1.5, 0.01, "Wind turbulence:");
		Self::drag_n_update(ui, &mut self.wind_power,		0.0..=0.05,  0.0005, "Wind power:");
	}

	pub fn ui(&mut self, ui: &mut Ui) {
		self.surface_ui(ui);
		ui.separator();
		self.spring_ui(ui);
		ui.separator();
		self.wind_ui(ui);
	}
	
	pub fn drag_n_update<Num: Numeric>(ui: &mut Ui, val: &mut Num, range: RangeInclusive<Num>, speed: f64, label: &str) {
		ui.label(label);
		ui.add(DragValue::new(val).clamp_range(range).speed(speed));
	}
}