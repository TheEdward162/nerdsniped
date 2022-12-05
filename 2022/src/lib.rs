use std::{path::{PathBuf, Path}, io::Read};

use anyhow::Context;

pub fn setup_logger(level: log::Level) -> anyhow::Result<()> {
	use edwardium_logger::{
		Logger,
		targets::stderr::StderrTarget
	};

	let logger = Logger::new(
		StderrTarget::new(level, Default::default()),
		std::time::Instant::now()
	);
	logger.init_boxed().context("Could not initialize logger")?;

	Ok(())
}

pub struct Cli {
	pub input: PathBuf,
	pub log_level: log::Level
}
pub fn parse_cli() -> anyhow::Result<Cli> {
	let mut it = std::env::args().skip(1);

		let mut log_level: Option<log::Level> = None;
		let mut input: Option<PathBuf> = None;

		while let Some(arg) = it.next() {
			match arg.as_str() {
				"--log-level" => {
					log_level = Some(
						match it.next().context("--log-level requires a value")?.as_str() {
							"trace" => log::Level::Trace,
							"debug" => log::Level::Debug,
							"info" => log::Level::Info,
							"warn" => log::Level::Warn,
							"error" => log::Level::Error,
							v => anyhow::bail!("Invalid --log-level value: {}", v)
						}
					);
				},
				new_input if input.is_none() => { input = Some(PathBuf::from(new_input)); }
				v => anyhow::bail!("Unknown argument: {}", v)
			}
		}

		Ok(Cli {
			input: input.unwrap(),
			log_level: log_level.unwrap_or(log::Level::Info)
		})
}

pub fn read_file(path: &Path) -> anyhow::Result<impl Read> {
	use std::fs::OpenOptions;

	let file = OpenOptions::new().read(true).open(path)?;
	
	Ok(file)
}

pub fn initialize() -> anyhow::Result<impl Read> {
	let cli = parse_cli().context("Failed to parse CLI")?;
	setup_logger(cli.log_level).context("Failed to set up logger")?;
	let file = read_file(&cli.input).context("Failed to open input file")?;

	Ok(file)
}
