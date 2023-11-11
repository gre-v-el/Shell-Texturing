mod furry_mesh;
mod camera;
mod state;

use egui_macroquad::macroquad;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("shell")]
async fn main() {

	let mut state = State::new();

	loop {
		clear_background(BLACK);
		
		state.draw();

		next_frame().await;
	}
}
