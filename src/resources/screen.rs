use crate::{
	components::pos::Pos, jitter::JitterFn, text::StyledText, FONT_RATIO,
	FONT_SIZE, PADDING, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use macroquad::{color::Color, prelude::*};

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

pub const GLYPH_DEFAULT: GlyphOptions = GlyphOptions {
	color: WHITE,
	background: BLANK,
	jitter: Jitter::Constant(0.0, 0.0),
	jitter_glyph: true,
	jitter_bg: false,
};

impl Default for GlyphOptions {
	fn default() -> Self {
		GLYPH_DEFAULT
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Jitter {
	Constant(f32, f32),
	Fn(JitterFn),
}

impl Default for Jitter {
	fn default() -> Self {
		Self::Constant(0.0, 0.0)
	}
}

#[derive(Debug)]
pub struct Screen {
	pub glyphs: [[Glyph; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
	pub fn new() -> Self {
		Self {
			glyphs: [[Glyph::default(); SCREEN_WIDTH]; SCREEN_HEIGHT],
		}
	}

	pub fn write(&mut self, pos: &Pos, text: &StyledText) {
		let mut progress_x = 0;
		let mut progress_y = 0;

		for (i, c) in text.source.chars().enumerate() {
			if c == '\n' {
				progress_y += 1;
				progress_x = 0;
			} else {
				if pos.x + progress_x >= SCREEN_WIDTH
					|| pos.y + progress_y >= SCREEN_HEIGHT
				{
					progress_x += 1;
					continue;
				}
				self.glyphs[pos.y + progress_y][pos.x + progress_x] = Glyph {
					ch: c,
					options: *text.style_at(i),
				};
				progress_x += 1;
			}
		}
	}

	pub fn writeo(
		&mut self,
		(x, y): (usize, usize),
		s: &str,
		options: &GlyphOptions,
	) {
		if y >= SCREEN_HEIGHT {
			return;
		}
		let mut progress_x = 0;
		let mut y = y;
		for (i, c) in s.chars().enumerate() {
			if x + i >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
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
		self.glyphs = [[Glyph::default(); SCREEN_WIDTH]; SCREEN_HEIGHT]
	}

	pub fn dimensions() -> Pos<f32> {
		let font_size = FONT_SIZE as f32;
		Pos::new(
			SCREEN_WIDTH as f32 * font_size,
			SCREEN_HEIGHT as f32 * FONT_RATIO * font_size,
		)
	}

	pub fn position() -> Pos<f32> {
		let Pos {
			x: width,
			y: height,
		} = Self::dimensions();
		let padding = PADDING as f32;
		let top_right_x = padding.max((screen_width() / 2.0) - (width / 2.0));
		let top_right_y = padding.max((screen_height() / 2.0) - (height / 2.0));
		Pos::new(top_right_x, top_right_y)
	}

	pub fn draw(&self, font: Font) -> usize {
		let font_size = FONT_SIZE as f32;
		let Pos {
			x: top_right_x,
			y: top_right_y,
		} = Self::position();

		let mut draws_performed: usize = 0;
		for y in 0..SCREEN_HEIGHT {
			for x in 0..SCREEN_WIDTH {
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
					let xpos = x as f32 * font_size + top_right_x + jx;
					draw_rectangle(
						xpos,
						y as f32 * font_size * FONT_RATIO + top_right_y + jy,
						font_size,
						font_size * FONT_RATIO * 1.25,
						glyph.options.background,
					);
				}
			}
		}

		for y in 0..SCREEN_HEIGHT {
			for x in 0..SCREEN_WIDTH {
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

					let xpos = x as f32 * font_size + top_right_x + jx;
					draw_text_ex(
						s,
						xpos,
						(y + 1) as f32 * font_size * FONT_RATIO + top_right_y + jy,
						TextParams {
							font,
							font_size: FONT_SIZE,
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
