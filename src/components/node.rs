use super::pos::Pos;
use crate::{resources::input::Mouse, text::StyledText, TermScreen};
use legion::*;

pub struct Node {
	pub text: StyledText,
	pub pos: Pos<usize>,
	raw_cache: String,
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
		let mut text = StyledText::parse(text).unwrap().1;
		if wrapped {
			text = text.wrap(width);
		}
		let unstyled = text.unstyled();

		let (x, y) = unstyled.lines().fold((0, 0), |(x, y), line| {
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
			raw_cache: unstyled,
			hitbox: (pos, bl),
		}
	}

	pub fn mouse_is_over(&self, mouse: &Mouse) -> bool {
		mouse.in_box(self.hitbox.0, self.hitbox.1)
	}
}

#[system(for_each)]
pub fn draw_nodes(node: &Node, #[resource] screen: &mut TermScreen) {
	screen.write(&node.pos, &node.text);
}
