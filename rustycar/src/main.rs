use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "172.0.0.1:8080";

fn main() {
	let delay = Delay;
	let mut gpio = Gpio::new().unwrap();

	let tcpl = TcpListener::bind(ADDRESS).unwrap();

	loop {
		if let Ok((stream, _address)) = tcpl.accept() {
			eprintln!("{:?}", handle_stream(stream, &mut gpio).err());
		}
	}
}

const STOP: u8 = 0b0;
const LEFT: u8 = 0b0000_0001;
const RIGHT: u8 = 0b0000_0010;

const L_F: u8 = 5;
const L_B: u8 = 6;
const R_F: u8 = 23;
const R_B: u8 = 24;

const FREQUENCY: f64 = 100.0;

// Basic idea: send a byte containing the set of instructions, then the values of the instructions.
// Example:
// 0000 0011		0100 1001		0011 0001
// LEFT & RIGHT		v_L = 145		v_R = 96

fn handle_stream(mut stream: TcpStream, gpio: &mut Gpio) -> std::io::Result<()> {
	let mut pin_l_f = gpio.get(L_F).unwrap().into_output();
	let mut pin_l_b = gpio.get(L_B).unwrap().into_output();
	let mut pin_r_f = gpio.get(R_F).unwrap().into_output();
	let mut pin_r_b = gpio.get(R_B).unwrap().into_output();
	loop {
		let mut buf = [0u8; 1];
		let buf = &mut buf;
		stream.read_exact(buf)?;
		let header = buf[0];
		if header == STOP {
			break;
		}
		if (header & LEFT) != 0 {
			stream.read_exact(buf)?;
			handle_motor(&mut pin_l_f, &mut pin_l_b, magic_fn(buf[0])).ok();
		}
		if (header & RIGHT) != 0 {
			stream.read_exact(buf)?;
			handle_motor(&mut pin_r_f, &mut pin_r_b, magic_fn(buf[0])).ok();
		}
	}
	Ok(())
}

use rppal::gpio;

fn handle_motor(pin_f: &mut OutputPin, pin_b: &mut OutputPin, speed: f64) -> gpio::Result<()> {
	if speed == 0.0 {
		pin_f.set_low();
		pin_b.set_low();
	} else if speed > 0.0 {
		pin_b.set_low();
		pin_f.set_pwm_frequency(FREQUENCY, speed)?;
	} else if speed < 0.0 {
		pin_f.set_low();
		pin_b.set_pwm_frequency(FREQUENCY, -speed)?;
	}
	Ok(())
}

const MIN_DUTY: f64 = 0.3;

fn magic_fn(num: u8) -> f64 {
	let num = unsafe {
		let num = *(&num as *const u8 as *const i8);
		num
	};
	((num as f64) / std::i8::MAX as f64) * (1.0 - MIN_DUTY) + MIN_DUTY
}
