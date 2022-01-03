use bevy_ecs::schedule::Stage;
use macroquad::prelude::*;
use world::world;

use crate::screen::Screen;

mod screen;
mod ticker;
mod world;
mod components {
	pub mod components;
	pub mod cycler;
	pub mod node;
	pub mod pos;
}
mod input;
mod jitter;
mod text;

pub const SCREEN_WIDTH: usize = 80; // characters
pub const SCREEN_HEIGHT: usize = 40; // characters
pub const FONT_SIZE: u16 = 12;
pub const PADDING: u16 = 10; // pixels
pub const FONT_RATIO: f32 = 1.5;
pub const DEBUG: bool = true;

fn window_conf() -> Conf {
	let padding = PADDING as i32;
	let h = (SCREEN_HEIGHT as f32 * FONT_SIZE as f32 * FONT_RATIO).floor() as i32;
	Conf {
		window_title: "Term Game Lib".to_string(),
		window_width: SCREEN_WIDTH as i32 * FONT_SIZE as i32 + padding * 2,
		window_height: h + padding * 2,
		window_resizable: true,
		high_dpi: true,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let font = load_ttf_font_from_bytes(include_bytes!(
		"../resources/CascadiaCode-Regular.ttf"
	))
	.unwrap();
	let (mut world, mut schedule) = world();
	loop {
		{
			let mut screen = world.get_resource_mut::<Screen>().unwrap();
			screen.clear();
		}
		clear_background(BLACK);
		schedule.run(&mut world);
		let screen = world.get_resource::<Screen>().unwrap();
		let draws = screen.draw(font);
		if DEBUG {
			let debug = format!("{} - glyphs: {}", get_fps(), draws);
			draw_text(&debug, 10.0, 10.0, 8.0, WHITE);
		}
		next_frame().await;
	}
}
