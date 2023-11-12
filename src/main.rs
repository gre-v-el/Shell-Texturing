mod furry_mesh;
mod camera;
mod furry_material;
mod state;
mod spring;
mod params;

use std::env;

use egui_macroquad::macroquad;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("shell")]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut state = State::new();

	loop {
		clear_background(BLACK);
		
		state.draw();

		next_frame().await;
	}
}

/*
todo:
* triangulate messh instead of panicing
* add color palettes
* better light
* don't upload all uniforms each frame
* presets
*/