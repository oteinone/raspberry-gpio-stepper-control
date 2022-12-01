extern crate rppal;
use rppal::gpio::{Gpio, OutputPin, Level};
use std::{thread, time::Duration};
use log::*;

// Port numbers for control board ports 1-4
const PIN1: u8 = 2;
const PIN2: u8 = 3;
const PIN3: u8 = 4;
const PIN4: u8 = 17;

// Delay between commands to control board (motor specification states max 600 Hz so this should not be under 2!)
const DELAY: u8 = 5;

// How many steps is a full rotation
//const ROTATION_STEP_COUNT: u16 = 512;

// SEQUENCE Assumes that pins are connected right...
const SEQUENCE: [[u8;4];8] = [
	[1,0,0,0],
	[1,1,0,0],
	[0,1,0,0],
	[0,1,1,0],
	[0,0,1,0],
	[0,0,1,1],
	[0,0,0,1],
	[1,0,0,1]
];

// Represents the failures in GPIO
pub enum IOFailure {
	GenericError
}

// Represents rotation
// Forward == Clockwise, Backward == Counterclockwise
#[derive(PartialEq)]
pub enum Direction {
	Forward,
	Backward
}

// 512 steps for a revolution
pub fn rotate_steps(steps: u16, direction: Direction) {
	match get_ports() {
		Err(_) => error!("Help, I'm being oppressed!"),
		Ok(mut pins) => {
			info!("Rotating {} steps {}", steps, if direction == Direction::Forward { "Forward" } else { "Backward" });
			for step in 0..steps {
				if ((step + 1) % 10) == 0 {
					debug!("Running step {}", step + 1);
				}
				for sequence_step in 0..8 {
					run_step(&mut pins, if direction == Direction::Forward { sequence_step } else { 7 - sequence_step });
					thread::sleep(Duration::from_millis(DELAY.into()));			
				} 
			}
			info!("Rotating done");
			reset_pins(&mut pins);

		}
	}

}

pub fn degrees_to_steps(degrees: u16) -> u16 {
	return (degrees * 64) / 45; // (64/45) == (512/360)
}

fn run_step(pins: &mut [OutputPin;4], current_step: u8) {
	fn u8_to_level(value: u8) -> Level {
		return if value == 1 { 
			Level::High
		} else {
			Level::Low
		}
	}

	for i in 0..4 {
		pins[i].write(u8_to_level(SEQUENCE[usize::from(current_step)][i]));
	}
}

fn reset_pins(pins: &mut [OutputPin;4]) {
	info!("Resetting pins!");
	for i in 0..4 {
		pins[i].set_low();
	}
}

fn get_ports() -> Result<[OutputPin;4], IOFailure> {
	let gpio_init = Gpio::new();
	if !gpio_init.is_ok() {
		error!("GPIO library initialization failed! (GPIO general init)");
		return Err(IOFailure::GenericError);
	}
	let gpio = gpio_init.unwrap();
	let pin1 = gpio.get(PIN1);
	let pin2 = gpio.get(PIN2);
	let pin3 = gpio.get(PIN3);
	let pin4 = gpio.get(PIN4);

	if !(pin1.is_ok() && pin2.is_ok() && pin3.is_ok() && pin4.is_ok()) {
		error!("GPIO library initialization failed! (PIN init)");
		return Err(IOFailure::GenericError);
	}

	debug!("GPIO library was succesfully initiated!");
	return Ok([
		pin1.unwrap().into_output(),
		pin2.unwrap().into_output(),
		pin3.unwrap().into_output(),
		pin4.unwrap().into_output()
	]);		
}
