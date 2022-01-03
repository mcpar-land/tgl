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
	screen::{GlyphOptions, Jitter},
	Screen,
};

#[derive(Clone, Debug, PartialEq)]
pub struct StyledSpan {
	pub text: String,
	pub style: GlyphOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyledText(pub Vec<StyledSpan>);

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
		let (input, first_style) =
			map(opt(glyph_options), |o| o.unwrap_or_default())(input)?;
		let (input, first_text) = alt((take_until1("#["), rest))(input)?;
		let first_span = StyledSpan {
			text: first_text.to_string(),
			style: first_style,
		};
		let (input, mut pairs) = many0(map(
			pair(glyph_options, alt((take_until1("#["), rest))),
			|(style, text)| StyledSpan {
				text: text.to_string(),
				style,
			},
		))(input)?;

		pairs.insert(0, first_span);

		Ok((input, StyledText(pairs)))
	}

	pub fn unstyled(&self) -> String {
		let mut s = String::new();
		for span in &self.0 {
			s.push_str(span.text.as_str());
		}
		s
	}
	pub fn style_ranges(&self) -> Vec<std::ops::Range<usize>> {
		let mut indices = vec![0..self.0[0].text.len()];
		for i in 1..self.0.len() {
			indices
				.push(indices[i - 1].end..(indices[i - 1].end + self.0[i].text.len()));
		}
		indices
	}

	pub fn wrap(&self, width: usize) -> StyledText {
		let mut wrapped = self.unstyled();
		textwrap::fill_inplace(&mut wrapped, width);
		StyledText(
			self
				.0
				.iter()
				.map(|s| s.style.clone())
				.zip(self.style_ranges())
				.map(|(opts, range)| StyledSpan {
					text: wrapped[range].to_string(),
					style: opts,
				})
				.collect(),
		)
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
			StyledText(vec![StyledSpan {
				text: t.to_string(),
				style: GlyphOptions::default()
			}])
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
			StyledText(vec![
				StyledSpan {
					text: "This has some ".to_string(),
					style: GlyphOptions::default()
				},
				StyledSpan {
					text: "colored text ".to_string(),
					style: GlyphOptions {
						color: RED,
						..Default::default()
					}
				},
				StyledSpan {
					text: "inside.\n".to_string(),
					style: GlyphOptions::default()
				},
				StyledSpan {
					text: "And a newline!".to_string(),
					style: GlyphOptions {
						color: GOLD,
						jitter: Jitter::Fn(jitter_sin),
						..Default::default()
					}
				}
			])
		)
	}
	#[test]
	fn span_ranges() {
		let (rest, res) = StyledText::parse(
			"This has some #[red]colored text #[]inside.\n#[gold,sin]And a newline!",
		)
		.unwrap();
		assert_eq!(res.style_ranges(), vec![0..14, 14..27, 27..35, 35..49]);
	}
}
