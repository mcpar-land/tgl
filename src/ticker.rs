use crate::{
	components::{label::Label, node::Node},
	resources::delta_time::DeltaTime,
	text::StyledText,
};
use bevy_ecs::prelude::*;
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

pub fn run_tickers_sys(
	mut query: Query<(&mut Ticker, &mut Label)>,
	dt: Res<DeltaTime>,
) {
	for (mut ticker, mut label) in query.iter_mut() {
		if let Some(text) = ticker.text.clone() {
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
				label.text = StyledText {
					source: text.source[0..=ticker.tick_position].to_string(),
					styles: text.styles.clone(),
				}
			}
		} else {
			ticker.text = Some(label.text.clone());
			label.text = StyledText::empty();
		}
	}
}
