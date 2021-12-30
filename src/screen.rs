use std::os::windows::prelude::OsStrExt;

use macroquad::{
	color::Color,
	prelude::{draw_text_ex, Font, TextParams},
};

#[derive(Clone, Copy)]
pub struct Glyph {
	pub ch: char,
	pub color: Color,
}

pub struct Screen<const W: usize, const H: usize>(pub [[Glyph; W]; H]);

impl<const W: usize, const H: usize> Screen<W, H> {
	pub fn new() -> Self {
		Self(
			[[Glyph {
				ch: ' ',
				color: macroquad::color::WHITE,
			}; W]; H],
		)
	}

	pub fn write(&mut self, (x, y): (usize, usize), s: &str, color: Color) {
		if y >= H {
			return;
		}
		for (i, c) in s.chars().enumerate() {
			if x + i >= W {
				return;
			}
			self.0[y][x + i] = Glyph { ch: c, color };
		}
	}

	pub fn clear(&mut self) {
		self.0 = [[Glyph {
			ch: ' ',
			color: macroquad::color::WHITE,
		}; W]; H]
	}

	pub fn draw(&self, font_size: u16, font: Font) {
		for y in 0..H {
			for x in 0..W {
				let glyph = &self.0[y][x];
				let mut tmp = [0u8; 4];
				let s = glyph.ch.encode_utf8(&mut tmp);

				if glyph.ch != ' ' {
					draw_text_ex(
						s,
						x as f32 * font_size as f32,
						y as f32 * font_size as f32,
						TextParams {
							font,
							font_size,
							font_scale: 1.0,
							font_scale_aspect: 1.0,
							color: self.0[y][x].color,
						},
					);
				}
			}
		}
	}
}
