use macroquad::prelude::*;

use crate::screen::Screen;

mod screen;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const FONT_SIZE: u16 = 18;

#[macroquad::main("Term Game Lib")]
async fn main() {
	let font = load_ttf_font_from_bytes(include_bytes!(
		"../resources/CascadiaCode-Regular.ttf"
	))
	.unwrap();
	let mut screen = Screen::<SCREEN_WIDTH, SCREEN_HEIGHT>::new();

	loop {
		clear_background(BLACK);
		screen.write((8, 4), "Hello, world!", GREEN);
		screen.draw(FONT_SIZE, font);
		draw_text(&get_fps().to_string(), 10.0, 10.0, 8.0, WHITE);
		next_frame().await;
	}
}
