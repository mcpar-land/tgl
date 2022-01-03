use crate::{components::pos::Pos, TermScreen, SCREEN_HEIGHT, SCREEN_WIDTH};
use legion::*;
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub struct Mouse(pub Option<Pos<usize>>);

impl Mouse {
	pub fn in_box(&self, top_left: Pos<usize>, bottom_right: Pos<usize>) -> bool {
		if let Mouse(Some(pos)) = &self {
			pos.x >= top_left.x
				&& pos.y >= top_left.y
				&& pos.x <= bottom_right.x
				&& pos.y <= bottom_right.y
		} else {
			false
		}
	}
}

#[system]
pub fn calc_input(#[resource] mouse: &mut Mouse) {
	let root = TermScreen::position();
	let dimensions = TermScreen::dimensions();
	let bottom_right = root + dimensions;
	let m = mouse_position_local();
	let m = (Pos::new(m.x, m.y) + 1.0) * (1.0 / 2.5);
	let m = Pos::new(m.x * screen_width(), m.y * screen_height());
	if root.x > m.x
		|| root.y > m.y
		|| bottom_right.x < m.x
		|| bottom_right.y < m.y
	{
		mouse.0 = None;
	} else {
		let m = m - root;
		mouse.0 = Some(Pos::new(
			(m.x * (SCREEN_WIDTH as f32 / dimensions.x)).floor() as usize,
			(m.y * (SCREEN_HEIGHT as f32 / dimensions.y)).floor() as usize,
		));
	}
}
