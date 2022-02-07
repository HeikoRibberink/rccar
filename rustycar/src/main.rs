use rppal::gpio::OutputPin;
use rppal::gpio::Gpio;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "172.0.0.1:8080";


fn main() {
	let delay = Delay;
	let mut gpio = Gpio::new().unwrap();

	let tcpl = TcpListener::bind(ADDRESS).unwrap();

	loop {
		if let Ok((mut stream, _address)) = tcpl.accept() {
			handle_stream(stream, &mut gpio);
			todo!()
		}
	}
}

const LEFT: u8 = 0b0000_0001;
const RIGHT: u8 = 0b0000_0010;

const L_F: u8 = 5;
const L_B: u8 = 6;
const R_F: u8 = 23;
const R_B: u8 = 24;

const FREQUENCY: f64 = 100.0;

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
		if (header & LEFT) != 0 {
			stream.read_exact(buf)?;
			handle_motor(&mut pin_l_f, &mut pin_l_b, magic_fn(buf[0]))
		}
	}
	Ok(())
}

fn handle_motor(pin_f: &mut OutputPin, pin_b: &mut OutputPin, speed: f64) {
	todo!()
}

fn magic_fn(num: u8) -> f64 {
	let num = unsafe {let num = *(&num as *const u8 as *const i8); num};
	num as f64
}
