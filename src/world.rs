use legion::*;
use macroquad::prelude::*;

use crate::{screen::Screen, TermScreen, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH};

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
	screen.write((0, 0), "0", RED);
	screen.write((79, 39), "X", RED);

	screen.write((8, 4), "Hello, world!", GREEN);

	screen.write((20, 20), "This one has\na newline in it", BLUE);
}
