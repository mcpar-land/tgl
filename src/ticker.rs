use crate::{
	components::node::Node, resources::delta_time::DeltaTime, text::StyledText,
};
use legion::*;

// use crate::dialog::dialog::DialogFile;

pub struct Ticker {
	pub text: StyledText,
	pub delay: f32,
	tick_position: usize,
	timer: f32,
}

impl Ticker {
	pub fn new(text: StyledText) -> Self {
		Self {
			text,
			delay: 0.1,
			tick_position: 0,
			timer: 0.0,
		}
	}
}

#[system(for_each)]
pub fn run_tickers(
	ticker: &mut Ticker,
	node: &mut Node,
	#[resource] dt: &DeltaTime,
) {
	// if ticker.tick_position >= ticker.source.len() {
	// 	return;
	// }
	// ticker.timer += dt.0;
	// if ticker.timer > ticker.delay {
	// 	ticker.tick_position += 1;
	// 	ticker.timer = 0.0;

	// let unstyled = ticker.text.unstyled();
	// let mut chars = unstyled.chars().skip(ticker.tick_position);
	// while [Some('\n'), Some(' ')].contains(&chars.next()) {
	// 	ticker.tick_position += 1;
	// }
	// }
}
