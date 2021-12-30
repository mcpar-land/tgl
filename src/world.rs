use legion::*;
use macroquad::prelude::*;

use crate::{
	screen::{GlyphOptions, Jitter, Screen},
	TermScreen, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn world() -> (World, Resources, Schedule) {
	let world = World::default();

	let mut resources = Resources::default();
	resources.insert(Screen::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(
		PADDING as f32,
		PADDING as f32,
	));

	let schedule = Schedule::builder().add_system(do_stuff_system()).build();

	(world, resources, schedule)
}

#[system]
pub fn do_stuff(#[resource] screen: &mut TermScreen) {
	screen.writec((0, 0), "0", RED);
	screen.writec((79, 39), "X", RED);

	screen.writeo(
		(8, 4),
		"Hello, world!",
		GlyphOptions {
			background: GRAY,
			..Default::default()
		},
	);

	screen.writeo(
		(20, 20),
		"This one, here? It has\na newline in it.",
		GlyphOptions {
			background: RED,
			jitter: Jitter::Fn(jitter_sin),
			..Default::default()
		},
	);
}

pub fn jitter_sin((x, y): (usize, usize), time: f64) -> (f32, f32) {
	(
		0.0,
		((time as f32 + x as f32 + y as f32 * 20.0) * 20.0).sin(),
	)
}
