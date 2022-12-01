extern crate rppal;
use rppal::gpio::{Gpio, OutputPin, Level};
use std::{thread, time::Duration};
use log::*;

// Delay between commands to control board (motor specification states max 600 Hz so this should not be under 2!)
const DELAY: u8 = 5;

// How many cycles of the motor is a full rotation of the shaft
//const ROTATION_CYCLE_COUNT: u16 = 512;

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

// Rotates the motor 
pub fn rotate_degrees(degrees: u16, direction: Direction, io_pins: [u8;4]) {
	let cycles: u16 = degrees_to_cycles(degrees);
	match get_ports(io_pins) {
		Err(_) => error!("Help, I'm being oppressed!"),
		Ok(mut pins) => {
			info!("Rotating {} steps {}", cycles, if direction == Direction::Forward { "Forward" } else { "Backward" });
			for step in 0..cycles {
				if ((step + 1) % 10) == 0 {
					debug!("Running step {}", step + 1);
				}
				for sequence_step in 0..8 {
					run_cycle(&mut pins, if direction == Direction::Forward { sequence_step } else { 7 - sequence_step });
					thread::sleep(Duration::from_millis(DELAY.into()));			
				} 
			}

			info!("Rotating done");
			reset_pins(&mut pins);
		}
	}
}

// Converts degrees to cycles (cycle = 1 revolution of the motor component)
fn degrees_to_cycles(degrees: u16) -> u16 {
	return (degrees * 64) / 45; // (64/45) == (512/360)
}

// Toggles one cycle of the motor (1/512th revolution of the shaft)
fn run_cycle(pins: &mut [OutputPin;4], current_step: u8) {
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

// Resets all involved pins to LOW
fn reset_pins(pins: &mut [OutputPin;4]) {
	info!("Resetting pins!");
	for i in 0..4 {
		pins[i].set_low();
	}
}

// Gets output ports for motor control
fn get_ports(io_pins: [u8;4]) -> Result<[OutputPin;4], IOFailure> {
	let gpio_init = Gpio::new();
	if !gpio_init.is_ok() {
		error!("GPIO library initialization failed! (GPIO general init)");
		return Err(IOFailure::GenericError);
	}
	let gpio = gpio_init.unwrap();
	let pin1 = gpio.get(io_pins[0]);
	let pin2 = gpio.get(io_pins[1]);
	let pin3 = gpio.get(io_pins[2]);
	let pin4 = gpio.get(io_pins[3]);

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
