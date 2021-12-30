use macroquad::prelude::*;

use crate::screen::Screen;

mod screen;

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 40;
pub const FONT_SIZE: u16 = 12;
pub const PADDING: u16 = 10;
pub const FONT_RATIO: f32 = 1.5;

fn window_conf() -> Conf {
	let padding = (PADDING * 2) as i32;
	let h = (SCREEN_HEIGHT as f32 * FONT_SIZE as f32 * FONT_RATIO).floor() as i32;
	Conf {
		window_title: "Term Game Lib".to_string(),
		window_width: SCREEN_WIDTH as i32 * FONT_SIZE as i32 + padding,
		window_height: h + padding,
		window_resizable: false,
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
	let mut screen =
		Screen::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(PADDING as f32, PADDING as f32);

	let mut picker = 0;
	let one_at_a_time = "BINGUSMYBELOVED";
	for x in 0..SCREEN_WIDTH {
		for y in 0..SCREEN_HEIGHT {
			screen.write((x, y), ":", GRAY);
		}
	}
	loop {
		clear_background(BLACK);
		screen.write((0, 0), "0", RED);
		screen.write((79, 39), "X", RED);

		screen.write((8, 4), "Hello, world!", GREEN);
		screen.write(
			(13, 5),
			&format!("{}", one_at_a_time.chars().nth(picker).unwrap()),
			YELLOW,
		);
		screen.write((20, 20), "This one has\na newline in it", BLUE);
		let draws = screen.draw(FONT_SIZE, font);
		let debug = format!("{} - glyphs: {}", get_fps(), draws);
		draw_text(&debug, 10.0, 10.0, 8.0, WHITE);

		if picker >= one_at_a_time.len() - 1 {
			picker = 0;
		} else {
			picker += 1;
		}

		next_frame().await;
	}
}
