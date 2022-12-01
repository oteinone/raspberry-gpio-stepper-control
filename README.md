# About the project

I was interested in controlling something physical with my Raspberry Pi 400 GPIO. I am not a Python enthusiast so I decided to make an example implementation in Rust.

This project creates an executable that can be used to rotate a stepper motor 0-3600 degrees in either direction. Set speed, nothing fancy.


# Building the project

Basic `cargo build` should do the trick

# Commanding the stepper motor

To get help run the program without any arguments.

Note! The PINS are currently hardcoded, the connections should be made like the following or rotate.js should be changed:

*PIN - Driver Input*
- PIN2  - IN1
- PIN3  - IN2
- PIN4  - IN3
- PIN17 - IN4

(Also a 5V pin to + and a ground pin to -)

# Used hardware

- Raspberry Pi 400
- L293D driver board (fancy with blinking lights)
- ULN2003A stepper motor
- Power directly from Rasp 5V pin (not optimal but I did not drive heavy loads)

