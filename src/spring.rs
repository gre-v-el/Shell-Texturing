use egui_macroquad::macroquad::prelude::*;

pub struct Spring {
	pub pos: Vec3,
	pub vel: Vec3,
	pub k: f32,
}

impl Spring {
	pub fn new() -> Self {
		Self { pos: Vec3::ZERO, vel: Vec3::ZERO, k: 200.0 }
	}

	pub fn update(&mut self) {
		let dt = get_frame_time();

		let mut force = Vec3::ZERO;
		force += vec3(0.0, 0.0, -200.0); // gravity
		force -= self.k * self.pos;    // spring
		force -= self.vel * 4.0;	   // drag

		self.vel += force * dt;
		self.pos += self.vel * dt;
	}
}