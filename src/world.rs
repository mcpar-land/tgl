use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{
	components::{
		label::{draw_labels_sys, Label},
		node::Node,
		pos::Pos,
	},
	jitter::{jitter_noise, jitter_sin},
	resources::input::{calc_input_sys, Mouse},
	resources::{
		delta_time::DeltaTime,
		screen::{GlyphOptions, Jitter, Screen},
	},
	ticker::{run_tickers_sys, Ticker},
	DEBUG, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn world() -> (World, Schedule) {
	let mut world = World::default();

	world.insert_resource(Box::new(Screen::new()));
	world.insert_resource(Mouse::default());
	world.insert_resource(DeltaTime(0.0));
	for (x, y, s) in [
		(0, 0, "0"),
		(79, 39, "X"),
		(12, 12, "#[orange]hello there, buddy"),
		(17, 14, "#[sin]I'm a #[sin,pink]wiggly #[]homie"),
		(
			50,
			5,
			r#"
┳┳━━━━━━━━━━┓
┃you did it ┃
┃           ┃
┗━━━━━━━━━━━┛"#,
		),
	] {
		world
			.spawn()
			.insert(Node::new(x, y))
			.insert(Label::new(false, 0, s));
	}
	world.spawn().insert(Node::new(
			20,
			17)).insert(Label::new(true,
			30,
			"I'm a #[red,noise]jittering MENACE #[]to society. But don't let that fool you! I also like #[green]long walks on the beach.",
		)).insert(Ticker::new());

	let mut schedule = Schedule::default();

	let mut events = SystemStage::parallel();

	schedule.add_stage("events", events);

	let mut logic = SystemStage::parallel();

	logic.add_system(calc_input_sys.system());
	logic.add_system(run_tickers_sys.system());

	schedule.add_stage_after("events", "logic", logic);

	let mut draw = SystemStage::parallel();

	draw.add_system(draw_labels_sys.system());
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
