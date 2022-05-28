use async_std::stream;
use rustyline_async::{Readline, ReadlineError};

use std::{io::Write, time::Duration};

use futures::prelude::*;

#[async_std::main]
async fn main() -> Result<(), ReadlineError> {
	let mut periodic_timer1 = stream::interval(Duration::from_secs(2));
	let mut periodic_timer2 = stream::interval(Duration::from_secs(3));

	let (mut rl, mut stdout) = Readline::new("> ".to_owned()).unwrap();
	rl.set_max_history(10);

	simplelog::WriteLogger::init(
		log::LevelFilter::Debug,
		simplelog::Config::default(),
		stdout.clone(),
	)
	.unwrap();

	let mut running_first = true;
	let mut running_second = false;

	loop {
		futures::select! {
			_ = periodic_timer1.next().fuse() => {
				if running_first { writeln!(stdout, "First timer went off!")?; }
			}
			_ = periodic_timer2.next().fuse() => {
				if running_second { log::info!("Second timer went off!"); }
			}
			command = rl.readline().fuse() => match command {
				Ok(line) => {
					let line = line.trim();
					rl.add_history_entry(line.to_owned()).await;
					match line {
						"start task" => {
							writeln!(stdout, "Starting the task...")?;
							running_first = true;
						},
						"stop task" => {
							writeln!(stdout, "Stopping the task...")?;
							running_first = false;
						}
						"start logging" => {
							log::info!("Starting the logger...");
							running_second = true
						},
						"stop logging" => {
							log::info!("Stopping the logger...");
							running_second = false
						},
						"info" => {
							writeln!(stdout, r"
hello there
I use NixOS btw
its pretty cool
							")?;
						}
						_ => writeln!(stdout, "Command not found: \"{}\"", line)?,
					}
				},
				Err(ReadlineError::Eof) =>{ writeln!(stdout, "Exiting...")?; break },
				Err(ReadlineError::Interrupted) => writeln!(stdout, "^C")?,
				Err(err) => {
					writeln!(stdout, "Received err: {:?}", err)?;
					writeln!(stdout, "Exiting...")?;
					break;
				},
			}
		}
	}
	Ok(())
}
