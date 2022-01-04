use super::pos::Pos;
use crate::{resources::input::Mouse, text::StyledText, Screen};
use bevy_ecs::prelude::*;

pub struct Node {
	pub text: StyledText,
	pub pos: Pos<usize>,
	hitbox: (Pos<usize>, Pos<usize>),
}

impl Node {
	pub fn new(x: usize, y: usize, text: &str) -> Self {
		Self::_new(x, y, false, 0, text)
	}

	pub fn new_wrap(x: usize, y: usize, width: usize, text: &str) -> Self {
		Self::_new(x, y, true, width, text)
	}

	fn _new(x: usize, y: usize, wrapped: bool, width: usize, text: &str) -> Self {
		let pos = Pos::new(x, y);
		let mut text = StyledText::parse(text);
		if wrapped {
			text = text.wrap(width);
		}

		let (x, y) = text.source.lines().fold((0, 0), |(x, y), line| {
			if line.len() > x {
				(line.len(), y + 1)
			} else {
				(x, y + 1)
			}
		});
		let bl = Pos::new(x, y) + pos;

		Self {
			text,
			pos,
			hitbox: (pos, bl),
		}
	}

	pub fn mouse_is_over(&self, mouse: &Mouse) -> bool {
		mouse.in_box(self.hitbox.0, self.hitbox.1)
	}
}

pub fn draw_nodes_sys(query: Query<&Node>, mut screen: ResMut<Box<Screen>>) {
	for node in query.iter() {
		screen.write(&node.pos, &node.text);
	}
}
