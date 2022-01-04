use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{
	components::{
		node::{draw_nodes_sys, Node},
		pos::Pos,
	},
	input::{calc_input_sys, Mouse},
	jitter::{jitter_noise, jitter_sin},
	screen::{GlyphOptions, Jitter, Screen},
	DEBUG, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn world() -> (World, Schedule) {
	let mut world = World::default();

	//TODO for some reason this line causes a stack overflow? Neat?
	world.insert_resource(Box::new(Screen::new()));
	world.insert_resource(Mouse::default());
	for node in [
			Node::new(0, 0, "0"),
			Node::new(79, 39, "X"),
			Node::new(12, 12, "#[orange]hello there, buddy"),
			Node::new(17, 14, "#[sin]I'm a #[sin,pink]wiggly #[]homie"),
			Node::new_wrap(
				20,
				17,
				30,
				"I'm a #[red,noise]jittering MENACE #[]to society. But don't let that fool you! I also like #[green]long walks on the beach.",
			),
			Node::new(
				50,
				5,
				r#"
┳┳━━━━━━━━━━┓
┃you did it ┃
┃           ┃
┗━━━━━━━━━━━┛"#,
			)
		] {
			world.spawn().insert(node);
		}

	let mut schedule = Schedule::default();

	let mut events = SystemStage::parallel();

	schedule.add_stage("events", events);

	let mut logic = SystemStage::parallel();

	logic.add_system(calc_input_sys.system());

	schedule.add_stage_after("events", "logic", logic);

	let mut draw = SystemStage::parallel();

	draw.add_system(draw_nodes_sys.system());
	draw.add_system(print_debug_sys.system());

	schedule.add_stage_after("logic", "draw", draw);

	(world, schedule)
}

pub fn print_debug_sys(mouse: Res<Mouse>) {
	if DEBUG {
		if let Some(mouse) = mouse.0 {
			draw_text(&format!("Mouse: {}", mouse), 150.0, 10.0, 8.0, WHITE);
		}
	}
}
