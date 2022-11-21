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

#![warn(missing_docs)]

use std::path::PathBuf;

use asm::error::Error as AssemblerError;
use clap::{Arg, ArgAction, Command};
use sim::error::Error as SimulatorError;

mod error;

use error::Error;

fn run() -> Result<(), Error> {
	let matches = Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.subcommand_required(true)
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
	fern::Dispatch::new()
		.format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
		.chain(std::io::stderr())
		.level(log::LevelFilter::Debug)
		.apply()
		.unwrap_or_else(|err| {
			eprintln!("logger initialisation failed\n{:?}", err);
			std::process::exit(1)
		});

	match run() {
		Ok(_) => (),
		Err(e) => {
			eprintln!("{}", e);
			std::process::exit(1);
		},
	}
}
