//! Subtitle Tool to manipulate SRTs
//!
//! # Features
//! - Any processed SRT comes out UTF-8
//! - You can shift the SRT timecodes with `--shift`

use clap::Parser;
use label_logger::{info, success, warn};
use std::{
	fs,
	path::{Path, PathBuf},
};
use uchardet::detect_encoding_name;

/// CLI command options
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

	/// Specifies by how much we should shift subtitles
	#[clap(long, short)]
	pub(crate) shift: Option<String>,
}

fn main() -> eyre::Result<()> {
	let config = Config::parse();

	if !is_subtitle(&config.input) {
		warn!(
			"`{}` is not a subtitle file",
			config.input.to_string_lossy()
		);
	};

	if config.backup {
		backup_file(&config.input)?;
	}

	let mut content = convert_and_read(&config.input)?;

	if let Some(shft) = config.shift {
		shift(&mut content, shft)?;
	}

	// TODO: maybe warn on inplace edition of subtitles
	fs::write(config.output.unwrap_or(config.input), content)?;
	success!("Converted subtitle file to `UTF-8`");

	Ok(())
}

// TODO: extend to support other subtitle formats
/// Checks wether the given has a correct subtitle extention
fn is_subtitle(file: &Path) -> bool {
	file.extension().map_or(false, |ext| ext == "srt")
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
			info!("File is already `UTF-8`, skipping conversion");
			String::from_utf8(original_bytes)?
		}
		Ok(charset) => {
			info!("Detected encoding: {}", charset);
			iconv::decode(&original_bytes, &charset)?
		}
		Err(err) => eyre::bail!("Could not detect encoding: {err}"),
	};

	Ok(content)
}

/// Parses the input subtitle and shifts with whats specified in `--shift`
fn shift(content: &mut String, shift: String) -> eyre::Result<()> {
	todo!()
}
