//! ```text
//!  _    _ ______  _____          _____  _____
//! | |  | || ___ \|_   _|        |____ |/ __  \
//! | |  | || |_/ /  | |   ______     / /`' / /'
//! | |/\| ||  __/   | |  |______|    \ \  / /
//! \  /\  /| |     _| |_         .___/ /./ /___
//!  \/  \/ \_|     \___/         \____/ \_____/
//! ```
//!
//! # WPI-32
//!
//! Custom assembler and emulator for the
//! [RISC-V](https://en.wikipedia.org/wiki/RISC-V) CPU architecture
//!
//! TODO: **!!! T E S T S !!!**
//!
//! TODO: more detailed parser errors (maybe see if lexer errors can be
//!       improved as well but i can't rlly think of anything)

#![warn(missing_docs)]

use std::path::PathBuf;

use ansi_term::Colour::{Blue, Red, Yellow};
use asm::error::Error as AssemblerError;
use clap::{Arg, ArgAction, ArgMatches, Command};
use log::Level;
use sim::error::Error as SimulatorError;

mod error;

use error::Error;

fn run(matches: &ArgMatches) -> Result<(), Error> {
	if let Some(m) = matches.subcommand_matches("asm") {
		let input_path = m.get_one::<String>("file").map(PathBuf::from).unwrap();
		let output_path_raw = m.get_one::<String>("output_file").map(PathBuf::from);
		let output_path = match output_path_raw {
			Some(p) => p.with_extension("wpibin"),
			None => {
				let mut base = input_path.clone();
				base.set_extension("wpibin");
				base
			},
		};

		asm::assemble(&input_path, &output_path)?;
	} else if let Some(m) = matches.subcommand_matches("sim") {
		let input_path = m.get_one::<String>("file").map(PathBuf::from).unwrap();
		let ext = input_path.extension().map_or("", |ext| ext.to_str().unwrap());
		if ext != "wpibin" {
			return Err(Error::WrongFileType {
				found:    ext.to_string(),
				expected: "wpibin".to_string(),
			});
		}

		sim::simulate(&input_path)?;
	}

	Ok(())
}

fn main() {
	let matches = Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.subcommand_required(true)
		.arg(
			Arg::new("verbosity")
				.help("How verbose the output should be")
				.short('v')
				.long("verbose")
				.action(ArgAction::Count),
		)
		.subcommand(
			Command::new("asm")
				.about("Assemble a file into a binary")
				.arg_required_else_help(true)
				.arg(
					Arg::new("output_file")
						.help("The file to write the binary to")
						.short('o')
						.long("output")
						.action(ArgAction::Set),
				)
				.arg(Arg::new("file").help("The file to assemble").index(1).required(true)),
		)
		.subcommand(
			Command::new("sim")
				.about("Simulte the execution of a binary file")
				.arg_required_else_help(true)
				.arg(Arg::new("file").help("The binary to simulate").index(1).required(true)),
		)
		.get_matches();

	let verbosity = matches.get_count("verbosity");

	fern::Dispatch::new()
		.format(|out, msg, record| {
			let repr = format!("{}", msg);
			let repr = match record.level() {
				Level::Error => Red.bold().paint(repr).to_string(),
				Level::Warn => Yellow.bold().paint(repr).to_string(),
				Level::Info => Blue.bold().paint(repr).to_string(),
				_ => repr,
			};

			out.finish(format_args!("{}", repr))
		})
		.chain(std::io::stderr())
		.level(match verbosity {
			0 => log::LevelFilter::Warn,
			1 => log::LevelFilter::Info,
			_ => log::LevelFilter::Debug,
		})
		.apply()
		.unwrap_or_else(|err| {
			eprintln!("logger initialisation failed\n{:?}", err);
			std::process::exit(1)
		});

	match run(&matches) {
		Ok(_) => (),
		Err(e) => {
			eprintln!("{}", e);
			std::process::exit(1);
		},
	}
}
