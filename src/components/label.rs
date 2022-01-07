use crate::{
	resources::{input::Mouse, screen::Screen},
	text::StyledText,
};
use bevy_ecs::prelude::*;

use super::{node::Node, pos::Pos};

pub struct Label {
	pub text: StyledText,
	size: Pos<usize>,
}

impl Label {
	pub fn new(wrapped: bool, width: usize, text: &str) -> Self {
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

		Self {
			text,
			size: Pos::new(x, y),
		}
	}

	pub fn mouse_is_over(&self, node: &Node, mouse: &Mouse) -> bool {
		mouse.in_box(node.pos, node.pos + self.size)
	}
}

pub fn draw_labels_sys(
	query: Query<(&Node, &Label)>,
	mut screen: ResMut<Box<Screen>>,
) {
	for (node, label) in query.iter() {
		screen.write(&node.pos, &label.text);
	}
}
