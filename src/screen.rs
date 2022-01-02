use crate::{jitter::JitterFn, FONT_RATIO, FONT_SIZE};

use macroquad::{color::Color, prelude::*};

#[derive(Clone, Copy)]
pub struct Glyph {
	pub ch: char,
	pub options: GlyphOptions,
}

impl Default for Glyph {
	fn default() -> Self {
		Self {
			ch: ' ',
			options: GlyphOptions::default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct GlyphOptions {
	/// Color of the glyph
	pub color: Color,
	/// Color of the background rectangle
	pub background: Color,
	/// Jitter operation
	pub jitter: Jitter,
	/// Should jitter operation apply to the glyph? (default: true)
	pub jitter_glyph: bool,
	/// Should jitter operation apply to the background rectangle? (default: false)
	pub jitter_bg: bool,
}

impl Default for GlyphOptions {
	fn default() -> Self {
		Self {
			color: WHITE,
			background: BLANK,
			jitter: Jitter::default(),
			jitter_glyph: true,
			jitter_bg: false,
		}
	}
}

#[derive(Clone, Copy)]
pub enum Jitter {
	Constant(f32, f32),
	Fn(JitterFn),
}

impl Default for Jitter {
	fn default() -> Self {
		Self::Constant(0.0, 0.0)
	}
}

pub struct Screen<const W: usize, const H: usize> {
	pub glyphs: [[Glyph; W]; H],
	pub x: f32,
	pub y: f32,
}

impl<const W: usize, const H: usize> Screen<W, H> {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			glyphs: [[Glyph::default(); W]; H],
			x,
			y,
		}
	}

	pub fn write(&mut self, pos: (usize, usize), s: &str) {
		self.writeo(pos, s, &GlyphOptions::default());
	}

	pub fn writec(&mut self, pos: (usize, usize), s: &str, color: Color) {
		self.writeo(
			pos,
			s,
			&GlyphOptions {
				color,
				..Default::default()
			},
		);
	}

	pub fn writeo(
		&mut self,
		(x, y): (usize, usize),
		s: &str,
		options: &GlyphOptions,
	) {
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
				self.glyphs[y][x + progress_x] = Glyph {
					ch: c,
					options: options.clone(),
				};
				progress_x += 1;
			}
		}
	}

	pub fn clear(&mut self) {
		self.glyphs = [[Glyph::default(); W]; H]
	}

	pub fn draw(&self, font_size: u16, font: Font) -> usize {
		let mut draws_performed: usize = 0;

		for y in 0..H {
			for x in 0..W {
				let glyph = &self.glyphs[y][x];
				if glyph.options.background != BLANK {
					let (jx, jy) = if glyph.options.jitter_bg {
						match glyph.options.jitter {
							Jitter::Constant(jx, jy) => (jx, jy),
							Jitter::Fn(f) => f((x, y), get_time()),
						}
					} else {
						(0.0, 0.0)
					};
					let xpos = x as f32 * font_size as f32 + self.x + jx;
					draw_rectangle(
						xpos,
						y as f32 * font_size as f32 * FONT_RATIO + self.y + jy,
						FONT_SIZE as f32,
						FONT_SIZE as f32 * FONT_RATIO * 1.25,
						glyph.options.background,
					);
				}
			}
		}

		for y in 0..H {
			for x in 0..W {
				let glyph = &self.glyphs[y][x];
				if glyph.ch != ' ' {
					let mut tmp = [0u8; 4];
					let s = glyph.ch.encode_utf8(&mut tmp);

					let (jx, jy) = if glyph.options.jitter_glyph {
						match glyph.options.jitter {
							Jitter::Constant(jx, jy) => (jx, jy),
							Jitter::Fn(f) => f((x, y), get_time()),
						}
					} else {
						(0.0, 0.0)
					};

					let xpos = x as f32 * font_size as f32 + self.x + jx;
					draw_text_ex(
						s,
						xpos,
						(y + 1) as f32 * font_size as f32 * FONT_RATIO + self.y + jy,
						TextParams {
							font,
							font_size,
							font_scale: 1.0,
							font_scale_aspect: 1.0,
							color: self.glyphs[y][x].options.color,
						},
					);
					draws_performed += 1;
				}
			}
		}
		draws_performed
	}
}
