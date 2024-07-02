#![allow(dead_code, unused_variables)]

use std::{
	error::Error,
	fs::{File, OpenOptions},
	io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct System {
	name: String,
}

impl System {
	fn new() -> System {
		System {
			name: "Example System".to_owned(),
		}
	}

	fn rename(&mut self, name: String) {
		self.name = name
	}
}

fn main() {
	// gaslight the api into thinking we have already made the cli :3
	let database_location = "test_system/system.json".to_owned();

	match run(database_location) {
		Ok(()) => (),
		Err(e) => println!("uh ohhh!! {e}"),
	}
}

fn run(database_location: String) -> Result<(), Box<dyn Error>> {
	let mut system = load(&database_location)?;

	let system_name = system.name.clone();

	system.rename(system_name + " :3");

	println!("{:?}", system);

	save(&system, database_location)?;

	Ok(())
}

// Searialise system data
fn load(database_location: &String) -> Result<System, Box<dyn Error>> {
	let mut database = File::open(database_location)?;

	let mut buf = String::new();
	database.read_to_string(&mut buf)?;

	Ok(serde_json::from_str(&buf)?)
}

fn save(system: &System, database_location: String) -> Result<(), Box<dyn Error>> {
	let system_data = serde_json::to_string_pretty(system)?;
	let buf = system_data.as_bytes();

	let mut database = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(database_location)?;

	database.write(buf)?;

	Ok(())
}
