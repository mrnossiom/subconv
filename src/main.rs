#![doc = include_str!("../README.md")]

use clap::Parser;
use label_logger::{error, info, success};
use std::{
	fs,
	path::{Path, PathBuf},
	process,
};
use subconv::subrip;
use uchardet::detect_encoding_name;

/// `subconv` cli options
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
	/// Path to input subtitle to manipulate
	#[clap(long, short)]
	pub(crate) input: PathBuf,

	/// Path where subtitle should be written
	/// If not specified, defaults to input subtitle for inplace editing
	#[clap(long, short)]
	pub(crate) output: Option<PathBuf>,

	/// If enabled, input file will be written to `{name}.bak`
	#[clap(long, short)]
	pub(crate) backup: bool,

	// --- Parsing related
	/// Enable transformations on subtitles by parsing them
	#[clap(long, short)]
	pub(crate) parse: bool,

	/// Specifies by how much we should shift subtitles
	#[clap(long, short, requires("parse"), value_parser = subrip::parse_shift)]
	pub(crate) shift: Option<subrip::Shift>,
}

fn main() -> eyre::Result<()> {
	let config = Config::parse();

	if subtitle_format(&config.input).is_none() {
		error!(
			"`{}` is not among supported subtitle formats",
			config.input.display()
		);
		process::exit(1);
	};

	if config.backup {
		backup_file(&config.input)?;
	}

	let mut content = convert_and_read(&config.input)?;

	if config.parse {
		info!(label: "Parsing", "subtitles");
		let mut subs = subrip::parse_subtitle_file(&content)?;

		if let Some(shft) = config.shift {
			info!(label: "Shifting", "subtitle by {shft}");
			subrip::shift(&mut subs, &shft);
		}

		content = subs.to_string();
	}

	// TODO: maybe warn on inplace edition of subtitles
	let path = config.output.unwrap_or(config.input);
	fs::write(&path, content)?;
	success!("converted/edited subtitle wrote to `{}`", path.display());

	Ok(())
}

/// Supported formats
enum SubFormat {
	/// `SubRip` which has the `.srt` extension
	///
	/// Parsing is straight-forward
	SubRip,
}

// TODO: extend to support other subtitle formats
/// Checks whether the given has a correct subtitle extension
fn subtitle_format(file: &Path) -> Option<SubFormat> {
	match file.extension()?.to_string_lossy().as_ref() {
		"srt" => Some(SubFormat::SubRip),
		_ => None,
	}

	// file.extension().map_or(false, |ext| ext == "srt")
}

/// Copies the given file to an adjacent one with and added `.bak`
fn backup_file(input: &Path) -> eyre::Result<()> {
	let backup_file = input.with_extension(format!(
		"{}.bak",
		input.extension().unwrap_or_default().to_string_lossy()
	));

	if backup_file.exists() {
		eyre::bail!("Backup file already exists");
	};

	fs::copy(input, &backup_file)?;

	Ok(())
}

/// Detects the input file encoding and converts it to UTF-8
fn convert_and_read(input: &Path) -> eyre::Result<String> {
	let original_bytes = fs::read(input)?;

	let content = match detect_encoding_name(&original_bytes) {
		Ok(s) if s == "UTF-8" => {
			info!(label: "Skipping", "conversion (file is already `UTF-8`)");
			String::from_utf8(original_bytes)?
		}
		Ok(charset) => {
			info!(label: "Converting", "detected encoding `{}`", charset);
			iconv::decode(&original_bytes, &charset)?
		}
		Err(err) => eyre::bail!("Could not detect encoding: {err}"),
	};

	Ok(content)
}
