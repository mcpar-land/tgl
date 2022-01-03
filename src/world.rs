use legion::*;
use macroquad::prelude::*;

use crate::{
	components::{
		node::{draw_nodes_system, Node},
		pos::Pos,
	},
	jitter::{jitter_noise, jitter_sin},
	resources::screen::{GlyphOptions, Jitter, Screen},
	resources::{
		delta_time::DeltaTime,
		input::{calc_input_system, Mouse},
	},
	TermScreen, DEBUG, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn world() -> (World, Resources, Schedule) {
	let mut world = World::default();

	let mut resources = Resources::default();
	resources.insert(Screen::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(
		PADDING as f32,
		PADDING as f32,
	));
	resources.insert(Mouse::default());
	resources.insert(DeltaTime(0.0));

	world.push(((), Node::new(0, 0, "0")));
	world.push(((), Node::new(79, 39, "X")));
	world.push(((), Node::new(12, 12, "#[orange]hello there, buddy")));
	world.push((
		(),
		Node::new(17, 14, "#[sin]I'm a #[sin,pink]wiggly #[]homie"),
	));
	world.push(((), Node::new_wrap(
		20,
		17,
		30,
		"I'm a #[red,noise]jittering MENACE #[]to society. But don't let that fool you! I also like #[green]long walks on the beach.",
	)));

	world.push((
		(),
		Node::new(
			50,
			5,
			r#"
┳┳━━━━━━━━━━┓
┃you did it ┃
┃           ┃
┗━━━━━━━━━━━┛"#,
		),
	));

	let schedule = Schedule::builder()
		.add_system(calc_input_system())
		.add_system(draw_nodes_system())
		.add_system(print_debug_system())
		.build();

	(world, resources, schedule)
}

#[system]
pub fn print_debug(#[resource] mouse: &Mouse) {
	if DEBUG {
		if let Mouse(Some(mouse)) = &mouse {
			draw_text(&format!("Mouse: {}", mouse), 150.0, 10.0, 8.0, WHITE);
		}
	}
}
