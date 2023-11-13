mod furry_mesh;
mod camera;
mod furry_material;
mod state;
mod spring;
mod params;

use std::env;

use egui_macroquad::{macroquad, egui::FontDefinitions};
use macroquad::prelude::*;
use state::State;

fn conf() -> Conf {
	Conf { 
		window_title: "Shell Textfurring".to_owned(), 
		window_width: 700, 
		window_height: 600, 
		high_dpi: false, 
		fullscreen: false, 
		sample_count: 16, 
		window_resizable: true, 
		..Default::default()
	}
}

#[macroquad::main(conf)]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	egui_macroquad::cfg(|ctx| {
		ctx.set_pixels_per_point(1.3);
	});

	let mut state = State::new();

	loop {		
		state.draw();
		next_frame().await;
	}
}

/*
todo:
* triangulate mesh instead of panicing
* don't upload all uniforms each frame
* presets
* settings.txt (fullscreen, samples, ui_scale)
*/