//! Subtitle Tool, convert and shift SRTs

use clap::Parser;
use iconv::copy;
use label_logger::{info, success, warn};
use std::{
	fs::{self, OpenOptions},
	path::{Path, PathBuf},
};
use temp_file::TempFileBuilder;
use uchardet::detect_encoding_name;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// CLI command options
struct Config {
	/// Path to subtitle to convert
	pub(crate) path: PathBuf,

	#[clap(long, default_value = "UTF-8")]
	/// Target encoding
	pub(crate) to: String,
}

fn main() -> eyre::Result<()> {
	let config = Config::parse();
	let backup_file = config.path.with_extension(format!(
		"{}.bak",
		config
			.path
			.extension()
			.unwrap_or_default()
			.to_string_lossy()
	));

	if !is_subtitle(&config.path) {
		warn!("Not a subtitle file");
	};

	let original_bytes = fs::read(&config.path)?;

	let charset = match detect_encoding_name(&original_bytes) {
		Ok(charset) => {
			eyre::ensure!(
				charset != config.to,
				"Subtitle file is already {}",
				config.to
			);

			info!("Detected encoding: {}", charset);

			charset
		}

		Err(err) => eyre::bail!("Could not detect encoding: {err}"),
	};

	if backup_file.exists() {
		let other_file = TempFileBuilder::new()
			.prefix("sub.")
			.suffix(".srt")
			.build()?;

		fs::copy(&backup_file, other_file.path())?;

		warn!(
			"Backup file already exists, moving it to: {}",
			other_file.path().display()
		);

		other_file.leak();
	};

	fs::copy(&config.path, &backup_file)?;

	let backup = OpenOptions::new().read(true).open(&backup_file)?;
	let file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(&config.path)?;

	copy(backup, file, &charset, &config.to)?;

	success!("Converted subtitle file to {}", config.to);

	Ok(())
}

// TODO: extend to support other subtitle formats
/// Checks wether the given has a correct subtitle extention
fn is_subtitle(file: &Path) -> bool {
	file.extension().map_or(false, |ext| ext == "srt")
}
