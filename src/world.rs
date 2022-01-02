use legion::*;
use macroquad::prelude::*;

use crate::{
	components::{
		label::{draw_labels_system, Label},
		pos::Pos,
	},
	jitter::{jitter_noise, jitter_sin},
	screen::{GlyphOptions, Jitter, Screen},
	TermScreen, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn world() -> (World, Resources, Schedule) {
	let mut world = World::default();

	let mut resources = Resources::default();
	resources.insert(Screen::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(
		PADDING as f32,
		PADDING as f32,
	));

	world.push((Pos::new(0usize, 0), Label::new("0")));
	world.push((Pos::new(79usize, 39), Label::new("X")));

	world.push((
		Pos::new(12usize, 12),
		Label::new_fmt(
			"hello there, buddy",
			&GlyphOptions {
				color: ORANGE,
				..Default::default()
			},
		),
	));

	world.push((
		Pos::new(17usize, 14),
		Label::new_fmt(
			"im a wiggly homie",
			&GlyphOptions {
				color: PINK,
				jitter: Jitter::Fn(jitter_sin),
				..Default::default()
			},
		),
	));

	world.push((
		Pos::new(10usize, 17),
		Label::new_fmt(
			"im a jittering MENACE to society",
			&GlyphOptions {
				color: RED,
				jitter: Jitter::Fn(jitter_noise),
				..Default::default()
			},
		),
	));

	let schedule = Schedule::builder().add_system(draw_labels_system()).build();

	(world, resources, schedule)
}

#[system]
pub fn do_stuff(#[resource] screen: &mut TermScreen) {
	// screen.writec((0, 0), "0", RED);
	// screen.writec((79, 39), "X", RED);

	// screen.writeo(
	// 	(8, 4),
	// 	"Hello, world!",
	// 	GlyphOptions {
	// 		background: GRAY,
	// 		..Default::default()
	// 	},
	// );

	// screen.writeo(
	// 	(20, 20),
	// 	"This one, here? It has\na newline in it.",
	// 	GlyphOptions {
	// 		background: RED,
	// 		jitter: Jitter::Fn(jitter_noise),
	// 		..Default::default()
	// 	},
	// );
}
