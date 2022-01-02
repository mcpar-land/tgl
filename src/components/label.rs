use crate::{screen::GlyphOptions, TermScreen};

use legion::*;

use super::pos::Pos;

pub struct Label {
	pub text: String,
	pub options: GlyphOptions,
}

impl Label {
	pub fn new(text: &str) -> Self {
		Self::new_fmt(text, &GlyphOptions::default())
	}
	pub fn new_fmt(text: &str, options: &GlyphOptions) -> Self {
		Self {
			text: text.to_string(),
			options: options.clone(),
		}
	}
}

#[system(for_each)]
pub fn draw_labels(
	pos: &Pos<usize>,
	label: &Label,
	#[resource] screen: &mut TermScreen,
) {
	screen.writeo((pos.x, pos.y), &label.text, &label.options);
}
