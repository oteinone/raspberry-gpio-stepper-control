use crate::rotate::*;
use std::env;

mod rotate;

fn main() {
	let args : Vec<String> = env::args().collect();

	// Check arg count and first argument	
	if args.len() < 4 || args[1].to_lowercase() != "rotate" {
		print_help();
		return;
	}
	
	// Parse rotation
	let degrees = match args[2].parse::<i32>() {
		Ok(deg) => deg,
		Err(_) => -1
	};

	if degrees < 0 || degrees > 3600  {
		print_help();
		return;
	}

	let steps = match degrees {
		360 => 512,
		270 => 384,
		180 => 256,
		90 => 128,
		_ => (512 / 360) * degrees
	};

	match args[3].as_str() {
		"forward" | "f" => rotate_steps(steps.try_into().unwrap(), Direction::Forward),
		"backward" | "b" => rotate_steps(steps.try_into().unwrap(), Direction::Backward),
		_ => print_help()
	}

	//rotate_steps(128, Direction::Forward);
	//rotate_steps(128, Direction::Backward);

}

fn print_help() {
	println!("Usage:");
	println!("rust-gpio rotate [degrees] [forward/backward]");
	println!("[degrees]: number (e.g. 360), min. 0, max. 3600");
	println!("[forward/backward]: forward (or 'f') for clockwise rotation, backward (or 'b') for counter-clockwise rotation");
}
