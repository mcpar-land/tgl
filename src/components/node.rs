use super::pos::Pos;
use crate::{resources::input::Mouse, text::StyledText, Screen};
use bevy_ecs::prelude::*;

pub struct Node {
	pub pos: Pos<usize>,
}

impl Node {
	pub fn new(x: usize, y: usize) -> Self {
		Self {
			pos: Pos::new(x, y),
		}
	}
}
