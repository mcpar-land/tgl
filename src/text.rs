use nom::{
	branch::alt,
	bytes::complete::{tag, take_until1},
	character::complete::alpha1,
	combinator::{map, opt, rest},
	multi::{many0, separated_list0},
	sequence::{delimited, pair},
	Finish, IResult,
};

use crate::{
	components::pos::Pos,
	jitter::{jitter_noise, jitter_sin},
	resources::screen::{GlyphOptions, Jitter, GLYPH_DEFAULT},
	TermScreen,
};

#[derive(Clone, Debug, PartialEq)]
pub struct StyledText {
	pub source: String,
	pub styles: Vec<(usize, GlyphOptions)>,
}

fn glyph_options(input: &str) -> IResult<&str, GlyphOptions> {
	use macroquad::prelude::*;
	let (input, tags) =
		delimited(tag("#["), separated_list0(tag(","), alpha1), tag("]"))(input)?;
	let mut opts = GlyphOptions::default();
	for tag in tags {
		match tag {
			"red" => {
				opts.color = RED;
			}
			"green" => {
				opts.color = GREEN;
			}
			"blue" => {
				opts.color = BLUE;
			}
			"beige" => {
				opts.color = BEIGE;
			}
			"black" => {
				opts.color = BLACK;
			}
			"blank" => {
				opts.color = BLANK;
			}
			"brown" => {
				opts.color = BROWN;
			}
			"darkblue" => {
				opts.color = DARKBLUE;
			}
			"darkbrown" => {
				opts.color = DARKBROWN;
			}
			"darkgray" => {
				opts.color = DARKGRAY;
			}
			"darkgreen" => {
				opts.color = DARKGREEN;
			}
			"darkpurple" => {
				opts.color = DARKPURPLE;
			}
			"gold" => {
				opts.color = GOLD;
			}
			"gray" => {
				opts.color = GRAY;
			}
			"lightgray" => {
				opts.color = LIGHTGRAY;
			}
			"lime" => {
				opts.color = LIME;
			}
			"magenta" => {
				opts.color = MAGENTA;
			}
			"maroon" => {
				opts.color = MAROON;
			}
			"orange" => {
				opts.color = ORANGE;
			}
			"pink" => {
				opts.color = PINK;
			}
			"purple" => {
				opts.color = PURPLE;
			}
			"skyblue" => {
				opts.color = SKYBLUE;
			}
			"violet" => {
				opts.color = VIOLET;
			}
			"white" => {
				opts.color = WHITE;
			}
			"yellow" => {
				opts.color = YELLOW;
			}
			// backgrounds
			"bgred" => {
				opts.background = RED;
			}
			"bggreen" => {
				opts.background = GREEN;
			}
			"bgblue" => {
				opts.background = BLUE;
			}
			"bgbeige" => {
				opts.background = BEIGE;
			}
			"bgblack" => {
				opts.background = BLACK;
			}
			"bgblank" => {
				opts.background = BLANK;
			}
			"bgbrown" => {
				opts.background = BROWN;
			}
			"bgdarkblue" => {
				opts.background = DARKBLUE;
			}
			"bgdarkbrown" => {
				opts.background = DARKBROWN;
			}
			"bgdarkgray" => {
				opts.background = DARKGRAY;
			}
			"bgdarkgreen" => {
				opts.background = DARKGREEN;
			}
			"bgdarkpurple" => {
				opts.background = DARKPURPLE;
			}
			"bggold" => {
				opts.background = GOLD;
			}
			"bggray" => {
				opts.background = GRAY;
			}
			"bglightgray" => {
				opts.background = LIGHTGRAY;
			}
			"bglime" => {
				opts.background = LIME;
			}
			"bgmagenta" => {
				opts.background = MAGENTA;
			}
			"bgmaroon" => {
				opts.background = MAROON;
			}
			"bgorange" => {
				opts.background = ORANGE;
			}
			"bgpink" => {
				opts.background = PINK;
			}
			"bgpurple" => {
				opts.background = PURPLE;
			}
			"bgskyblue" => {
				opts.background = SKYBLUE;
			}
			"bgviolet" => {
				opts.background = VIOLET;
			}
			"bgwhite" => {
				opts.background = WHITE;
			}
			"bgyellow" => {
				opts.background = YELLOW;
			}

			// jitters
			"sin" => {
				opts.jitter = Jitter::Fn(jitter_sin);
			}
			"noise" => {
				opts.jitter = Jitter::Fn(jitter_noise);
			}
			"bgjitter" => {
				opts.jitter_bg = true;
			}
			"nojitterg" => {
				opts.jitter_glyph = false;
			}
			_ => {}
		}
	}
	Ok((input, opts))
}

impl StyledText {
	pub fn parse(input: &str) -> IResult<&str, Self> {
		let (input, first_style) = opt(glyph_options)(input)?;
		let (input, first_text) = alt((take_until1("#["), rest))(input)?;
		let (input, pairs): (&str, Vec<(GlyphOptions, &str)>) =
			many0(pair(glyph_options, alt((take_until1("#["), rest))))(input)?;

		let mut source = first_text.to_string();
		let mut styles = first_style.map_or_else(|| Vec::new(), |v| vec![(0, v)]);
		for (i, (style, s)) in pairs.into_iter().enumerate() {
			styles.push((source.len(), style));
			source.push_str(s);
		}

		Ok((input, StyledText { source, styles }))
	}

	pub fn style_at(&self, index: usize) -> &GlyphOptions {
		if self.styles.len() == 0 || index < self.styles[0].0 {
			&GLYPH_DEFAULT
		} else if self.styles.len() == 1 {
			&self.styles[0].1
		} else {
			let mut i = self.styles.len() - 1;
			while self.styles[i].0 > index && i > 0 {
				i -= 1;
			}
			&self.styles[i].1
		}
	}

	pub fn wrap(&self, width: usize) -> StyledText {
		let mut wrapped = self.source.clone();
		textwrap::fill_inplace(&mut wrapped, width);
		StyledText {
			source: wrapped,
			styles: self.styles.clone(),
		}
	}

	pub fn slice(&self, slice: std::ops::Range<usize>) -> StyledText {
		todo!();
	}
}

#[cfg(test)]
mod test {
	use macroquad::prelude::{GOLD, RED};

	use super::*;

	#[test]
	fn parse_default() {
		let t = "This is some default styled text";
		let (rest, res) = StyledText::parse(t).unwrap();
		assert_eq!(rest, "");
		assert_eq!(
			res,
			StyledText {
				source: t.to_string(),
				styles: Vec::new()
			}
		);
	}
	#[test]
	fn parse_tags() {
		let (rest, res) = StyledText::parse(
			"This has some #[red]colored text #[]inside.\n#[gold,sin]And a newline!",
		)
		.unwrap();
		assert_eq!(rest, "");
		assert_eq!(
			res,
			StyledText {
				source: "This has some colored text inside.\nAnd a newline!"
					.to_string(),
				styles: vec![
					(
						14,
						GlyphOptions {
							color: RED,
							..Default::default()
						}
					),
					(27, GlyphOptions::default()),
					(
						35,
						GlyphOptions {
							color: GOLD,
							jitter: Jitter::Fn(jitter_sin),
							..Default::default()
						}
					),
				]
			}
		)
	}
	#[test]
	fn test_style_at() {
		let (rest, res) = StyledText::parse(
			"This has some #[red]colored text #[]inside.\n#[gold,sin]And a newline!",
		)
		.unwrap();
		for i in 0..14 {
			assert_eq!(res.style_at(i), &GLYPH_DEFAULT);
		}
		for i in 14..27 {
			assert_eq!(
				res.style_at(i),
				&GlyphOptions {
					color: RED,
					..Default::default()
				}
			);
		}
		for i in 27..35 {
			assert_eq!(res.style_at(i), &GLYPH_DEFAULT);
		}
		for i in 35..res.source.len() {
			assert_eq!(
				res.style_at(i),
				&GlyphOptions {
					color: GOLD,
					jitter: Jitter::Fn(jitter_sin),
					..Default::default()
				}
			);
		}
	}
}
