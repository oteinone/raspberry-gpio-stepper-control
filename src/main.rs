use crate::rotate::*;
use std::env;
use log::*;

mod rotate;

fn main() {
	env_logger::init();
	
	let args : Vec<String> = env::args().collect();

	// Check arg count and first argument	
	if args.len() < 4 {
		warn!("Not enough arguments, expected 3 arguments");
		print_help();
		return;
	}
	
	if args[1].to_lowercase() != "rotate" {
		warn!("First argument should be 'rotate'");
		print_help();
		return;
	}
	
	// Parse rotation
	let degrees = match args[2].parse::<i32>() {
		Ok(deg) => deg,
		Err(_) => -1
	};
	
	//Validate rotation reading
	if degrees < 0 || degrees > 3600  {
		warn!("Expected rotation to be between 0 and 3600 degrees, parsed {} degrees", degrees);
		print_help();
		return;
	}

	match args[3].as_str() {
		// We can safely unwrap here because we've checked that the value is [0, 3600]
		"forward" | "f" => rotate_steps(degrees_to_steps(degrees.try_into().unwrap()), Direction::Forward),
		"backward" | "b" => rotate_steps(degrees_to_steps(degrees.try_into().unwrap()), Direction::Backward),
		_ => { warn!("Rotation direction was {} which is not a valid value", args[3]); print_help(); }
	}
}

fn print_help() {
	println!("Usage:");
	println!("rust-gpio rotate [degrees] [forward/backward]");
	println!("[degrees]: number (e.g. 360), min. 0, max. 3600");
	println!("[forward/backward]: forward (or 'f') for clockwise rotation, backward (or 'b') for counter-clockwise rotation");
}
