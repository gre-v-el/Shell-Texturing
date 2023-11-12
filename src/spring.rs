use egui_macroquad::macroquad::prelude::*;

use crate::params::Params;

pub struct Spring {
	pub pos: Vec3,
	pub vel: Vec3,
}

impl Spring {
	pub fn new() -> Self {
		Self { pos: Vec3::ZERO, vel: Vec3::ZERO }
	}

	pub fn update(&mut self, params: &Params) {
		let dt = get_frame_time().min(0.1);

		let mut force = Vec3::ZERO;
		force += vec3(0.0, 0.0, -params.stiffness);	// gravity
		force -= params.stiffness * self.pos; 	   	// spring
		force -= self.vel * params.drag;		// drag

		self.vel += force * dt;
		self.pos += self.vel * dt;

		if self.pos.length_squared() > 36.0 {
			self.pos = self.pos.clamp_length_max(36.0);
		}
	}
}