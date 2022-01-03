use crate::{
	components::node::Node, resources::delta_time::DeltaTime, text::StyledText,
};
use legion::*;

// use crate::dialog::dialog::DialogFile;

pub struct Ticker {
	pub text: Option<StyledText>,
	pub delay: f32,
	tick_position: usize,
	timer: f32,
}

const TICKER_DELAY: f32 = 0.025;

impl Ticker {
	pub fn new() -> Self {
		Self {
			text: None,
			delay: TICKER_DELAY,
			tick_position: 0,
			timer: 0.0,
		}
	}
	pub fn start(&mut self, text: &str) {
		let text = StyledText::parse(text);
		self.text = Some(text);
		self.tick_position = 0;
		self.timer = 0.0;
	}
}

#[system(for_each)]
pub fn run_tickers(
	ticker: &mut Ticker,
	node: &mut Node,
	#[resource] dt: &DeltaTime,
) {
	if let Some(text) = &ticker.text {
		if ticker.tick_position >= text.source.len() {
			return;
		}
		ticker.timer += dt.0;
		if ticker.timer > ticker.delay {
			ticker.tick_position += 1;
			ticker.timer = 0.0;

			if ticker.tick_position >= text.source.len() {
				return;
			}

			let mut chars = text.source.chars().skip(ticker.tick_position);
			while [Some('\n'), Some(' ')].contains(&chars.next()) {
				ticker.tick_position += 1;
			}
			node.text = StyledText {
				source: text.source[0..=ticker.tick_position].to_string(),
				styles: text.styles.clone(),
			}
		}
	} else {
		ticker.text = Some(node.text.clone());
		node.text = StyledText::empty();
	}
}
