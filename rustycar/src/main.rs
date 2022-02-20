use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "192.168.2.60:8080";

fn main() {
	let _delay = Delay;
	let mut gpio = Gpio::new().unwrap();

	let tcpl = TcpListener::bind(ADDRESS).unwrap();
	let mut running = true;

	while running {
		if let Ok((stream, _address)) = tcpl.accept() {
			eprintln!("{:?}", handle_stream(stream, &mut gpio, &mut running).err());
		}
	}
}

const L_F: u8 = 5;
const L_B: u8 = 6;
const R_F: u8 = 23;
const R_B: u8 = 24;

const STOP: u8 = 255;
const LEFT: u8 = 1;
const RIGHT: u8 = 2;

fn handle_stream(mut stream: TcpStream, gpio: &mut Gpio, running: &mut bool) -> std::io::Result<()> {
	let mut pin_l_f = gpio.get(L_F).unwrap().into_output();
	let mut pin_l_b = gpio.get(L_B).unwrap().into_output();
	let mut pin_r_f = gpio.get(R_F).unwrap().into_output();
	let mut pin_r_b = gpio.get(R_B).unwrap().into_output();
	while *running {
		let buf = &mut [0u8; 2];
		stream.read(buf)?;
		let header = buf[0];
		let value = magic_fn(buf[1]);
		println!("Got: {:?}", buf);
		match header {
			STOP => {*running = false},
			LEFT => {handle_motor(&mut pin_l_f, &mut pin_l_b, value).unwrap();},
			RIGHT => {handle_motor(&mut pin_r_f, &mut pin_r_b, value).unwrap();},
			h => {eprintln!("Unexpected header from server: {} ", h)}
		}
	}
	Ok(())
}

use rppal::gpio;

const FREQUENCY: f64 = 100.0;

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
	} as f64;
	if num == 0.0 {return 0.0;}
	(num / std::i8::MAX as f64) * (1.0 - MIN_DUTY) + (MIN_DUTY * if num > 0.0 {1.0} else {-1.0})
}
