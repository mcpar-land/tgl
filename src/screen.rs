use crate::{FONT_RATIO, PADDING};
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

pub struct Screen<const W: usize, const H: usize> {
	pub glyphs: [[Glyph; W]; H],
	pub x: f32,
	pub y: f32,
}

impl<const W: usize, const H: usize> Screen<W, H> {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			glyphs: [[Glyph {
				ch: ' ',
				color: macroquad::color::WHITE,
			}; W]; H],
			x,
			y,
		}
	}

	pub fn write(&mut self, (x, y): (usize, usize), s: &str, color: Color) {
		if y >= H {
			return;
		}
		let mut progress_x = 0;
		let mut y = y;
		for (i, c) in s.chars().enumerate() {
			if x + i >= W || y >= H {
				return;
			}
			if c == '\n' {
				y += 1;
				progress_x = 0;
			} else {
				self.glyphs[y][x + progress_x] = Glyph { ch: c, color };
				progress_x += 1;
			}
		}
	}

	pub fn clear(&mut self) {
		self.glyphs = [[Glyph {
			ch: ' ',
			color: macroquad::color::WHITE,
		}; W]; H]
	}

	pub fn draw(&self, font_size: u16, font: Font) -> usize {
		let mut draws_performed: usize = 0;
		for y in 0..H {
			for x in 0..W {
				let glyph = &self.glyphs[y][x];
				let mut tmp = [0u8; 4];
				let s = glyph.ch.encode_utf8(&mut tmp);

				if glyph.ch != ' ' {
					draw_text_ex(
						s,
						x as f32 * font_size as f32 + self.x,
						(y + 1) as f32 * font_size as f32 * FONT_RATIO + self.y,
						TextParams {
							font,
							font_size,
							font_scale: 1.0,
							font_scale_aspect: 1.0,
							color: self.glyphs[y][x].color,
						},
					);
					draws_performed += 1;
				}
			}
		}
		draws_performed
	}
}
