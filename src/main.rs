mod furry_mesh;
mod camera;
mod furry_material;
mod state;
mod spring;
mod params;

use std::{env, fs};

use egui_macroquad::macroquad;
use macroquad::prelude::*;
use state::State;

fn conf() -> Conf {
	let mut fullscreen: bool = false;
	let mut samples = 16;

	if let Ok(str) = fs::read_to_string("settings.txt") {
		for mut line in str.lines() {
			line = line.trim();
			if let Some((mut key, mut value)) = line.split_once(": ") {
				key = key.trim();
				value = value.trim();

				match key {
					"fullscreen" => fullscreen = value == "true",
					"samples" => samples = value.parse::<i32>().unwrap_or(samples),
					_ => {}
				}
			}
		}
	}

	Conf { 
		window_title: "Shell Textfurring".to_owned(), 
		window_width: 900, 
		window_height: 700, 
		high_dpi: false,
		fullscreen, 
		sample_count: samples, 
		window_resizable: true, 
		..Default::default()
	}
}

#[macroquad::main(conf)]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut state = State::new();

	let mut scale = 1.3;
	if let Ok(str) = fs::read_to_string("settings.txt") {
		for mut line in str.lines() {
			line = line.trim();
			if let Some((mut key, mut value)) = line.split_once(": ") {
				key = key.trim();
				value = value.trim();

				if key == "scale" {
					scale = value.parse::<f32>().unwrap_or(scale);
				}
			}
		}
	}

	egui_macroquad::cfg(|ctx| {
		ctx.set_pixels_per_point(scale);
	});


	loop {		
		state.draw();
		next_frame().await;
	}
}

/*
todo:
* triangulate mesh instead of panicing
* don't upload all uniforms each frame
*/