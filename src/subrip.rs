//! Handle `SubRip` files and shift the timestamps

use nom::{
	self,
	bytes::complete::{tag, take_until},
	character::complete::one_of,
	character::complete::{newline, u64 as u64p},
	combinator::{eof, opt},
	multi::many0,
};
use std::fmt;

/// Parses time given by users via `--shift` flag
#[derive(Debug, Clone)]
pub struct Shift {
	/// Should the time be removed or added
	negative: bool,
	/// Value of the offset
	offset: Timestamp,
}

/// Parse a user provided shift argument
pub fn parse_shift(i: &str) -> eyre::Result<Shift> {
	let (_, shift) = parse_shift_(i).map_err(|e| eyre::eyre!("{e}"))?;
	Ok(shift)
}

/// Parse a user provided shift argument using nom
fn parse_shift_(i: &str) -> nom::IResult<&str, Shift> {
	let (i, sign) = opt(one_of("+-"))(i)?;
	let (i, offset) = parse_timestamp(i)?;
	let (i, _) = eof(i)?;

	Ok((
		i,
		Shift {
			negative: sign.is_some_and(|c| c == '-'),
			offset,
		},
	))
}

impl fmt::Display for Shift {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}{}",
			if self.negative { '-' } else { '+' },
			self.offset
		)
	}
}

/// A full subtitle file
pub struct Subtitles {
	/// List of individual subtitle blocks
	pile: Vec<Subtitle>,
}

impl ToString for Subtitles {
	fn to_string(&self) -> String {
		self.pile.iter().map(ToString::to_string).enumerate().fold(
			String::new(),
			|mut a, (i, b)| {
				// Add subtitle block number
				a.push_str(&(i + 1).to_string());
				a.push('\n');
				// Add subtitle content
				a.push_str(&b);
				a.push('\n');
				a
			},
		)
	}
}

/// A single subtitle block
struct Subtitle {
	/// Time when sub should be displayed
	start: Timestamp,
	/// Time when sub should not be displayed anymore
	end: Timestamp,
	/// Subtitle text
	content: String,
}

impl ToString for Subtitle {
	fn to_string(&self) -> String {
		format!("{} --> {}\n{}\n", self.start, self.end, self.content)
	}
}

/// Parse a single subtitle block into [`Subtitle`]
fn parse_subtitle_block(i: &str) -> nom::IResult<&str, Subtitle> {
	let (i, start) = parse_timestamp(i)?;
	let (i, _) = tag(" --> ")(i)?;
	let (i, end) = parse_timestamp(i)?;
	let (i, _) = newline(i)?;
	let (i, content) = take_until("\n\n")(i)?;
	let (i, _) = tag("\n\n")(i)?;

	Ok((
		i,
		Subtitle {
			start,
			end,
			content: content.to_owned(),
		},
	))
}

/// Timestamp in subtitle block header
///
/// Display is `HH:MM:SS,XXX`
#[derive(Debug, Clone)]
struct Timestamp(u64);

impl Timestamp {
	/// Shift a [`Timestamp`] by the amount parsed and stored in [`Shift`]
	fn shift_by(&mut self, shift: &Shift) {
		if shift.negative {
			self.0 -= shift.offset.0;
		} else {
			self.0 += shift.offset.0;
		}
	}
}

/// Parse a subtitle timestamp
fn parse_timestamp(i: &str) -> nom::IResult<&str, Timestamp> {
	let (i, hours) = u64p(i)?;
	let (i, _) = tag(":")(i)?;
	let (i, minutes) = u64p(i)?;
	let (i, _) = tag(":")(i)?;
	let (i, seconds) = u64p(i)?;
	let (i, _) = one_of(",.")(i)?;
	let (i, millis) = u64p(i)?;

	Ok((
		i,
		Timestamp((hours * (60 * 60 * 1000)) + (minutes * 60 * 1000) + (seconds * 1000) + millis),
	))
}

impl fmt::Display for Timestamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let hours = self.0 / (60 * 60 * 1000);
		let rest = self.0 % (60 * 60 * 1000);

		let minutes = rest / (60 * 1000);
		let rest = rest % (60 * 1000);

		let seconds = rest / 1000;
		let rest = rest % 1000;

		let millis = rest % 1000;

		write!(f, "{hours:02}:{minutes:02}:{seconds:02},{millis:03}")
	}
}

/// Parse an entire subtitle file
pub fn parse_subtitle_file(content: &str) -> eyre::Result<Subtitles> {
	let (_, subs) = parse_subtitle_file_(content).map_err(|e| eyre::eyre!("{e}"))?;
	Ok(subs)
}

/// Parse an entire subtitle file using nom
fn parse_subtitle_file_(i: &str) -> nom::IResult<&str, Subtitles> {
	let (i, subtitles) = many0(|i| {
		let (i, _id) = u64p(i)?;
		let (i, _) = newline(i)?;
		let (i, sub) = parse_subtitle_block(i).map_err(|e| dbg!(e))?;

		Ok((i, sub))
	})(i)?;
	let (i, _) = eof(i)?;

	Ok((i, Subtitles { pile: subtitles }))
}

/// Shifts subtitles with the value of `--shift`
pub fn shift(subtitles: &mut Subtitles, shift: &Shift) {
	subtitles.pile.iter_mut().for_each(|s| {
		s.start.shift_by(shift);
		s.end.shift_by(shift);
	});
}

#[cfg(test)]
mod tests {
	#[test]
	fn parse_single_subtitle_block() {
		todo!()
	}
}
