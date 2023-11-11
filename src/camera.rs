use std::f32::consts::PI;

use egui_macroquad::macroquad;
use macroquad::prelude::*;

pub struct OrbitCamera {
	pub center: Vec3,
	pub polar: f32,
	pub azimuth: f32,
	pub zoom: f32,
	
	pub rotate_sinsitivity: f32,
	pub last_mouse: Vec2,
}

impl OrbitCamera {
	pub fn update(&mut self, can_move: bool) {
		let mouse = Vec2::from(mouse_position()) / vec2(screen_width(), screen_width());
		let delta = mouse - self.last_mouse;

		if can_move {
			if is_mouse_button_down(MouseButton::Left) {
				self.azimuth -= delta.x * self.rotate_sinsitivity;
				self.polar += delta.y * self.rotate_sinsitivity;
	
				self.polar = self.polar.clamp(0.1, PI-0.1);
			}
			if is_mouse_button_down(MouseButton::Right) {
				let forward = -Vec3 { 
					x: self.zoom * self.polar.sin() * self.azimuth.cos(),
					y: self.zoom * self.polar.sin() * self.azimuth.sin(),
					z: self.zoom * self.polar.cos(),
				}; 
				let right = self.zoom*forward.cross(vec3(0.0, 0.0, 1.0)).normalize_or_zero();
				let up = self.zoom*right.cross(forward).normalize_or_zero();
	
				self.center += right * delta.x;
				self.center += up * delta.y;
			}
			self.zoom *= 0.999f32.powf(mouse_wheel().1);
		}

		self.last_mouse = mouse;
	}

	pub fn camera(&self) -> Camera3D {
		Camera3D {
			position: Vec3 { 
				x: self.zoom * self.polar.sin() * self.azimuth.cos(),
				y: self.zoom * self.polar.sin() * self.azimuth.sin(),
				z: self.zoom * self.polar.cos(),
			} + self.center,
			target: self.center,
			..Default::default()
		}
	}
}

impl Default for OrbitCamera {
	fn default() -> Self {
		OrbitCamera {
			azimuth: 0.0,
			center: vec3(0.0, 0.0, 0.0),
			last_mouse: Vec2::from(mouse_position()) / vec2(screen_width(), screen_width()),
			polar: 0.0,
			rotate_sinsitivity: 6.0,
			zoom: -20.0,
		}
	}
}