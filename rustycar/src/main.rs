use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

mod test;

const ADDRESS: &str = "192.168.2.60:8080";


fn main() {
	// test::servo();
	// test::min_volt_motor();
	car()
}

fn car() {
	let _delay = Delay;
	let mut gpio = Gpio::new().unwrap();

	let tcpl = TcpListener::bind(ADDRESS).unwrap();
	let mut running = true;

	while running {
		if let Ok((stream, _address)) = tcpl.accept() {
			// eprintln!("{:?}", handle_stream_tank(stream, &mut gpio, &mut running).err());
			eprintln!("{:?}", handle_stream_differential(stream, &mut gpio, &mut running).err());
		}
	}
}

const L_F: u8 = 17;
const L_B: u8 = 27;
const R_F: u8 = 23;
const R_B: u8 = 24;
const SERVO: u8 = 25;

const STOP: u8 = 255;
const LEFT: u8 = 1;
const RIGHT: u8 = 2;

fn handle_stream_tank(
	mut stream: TcpStream,
	gpio: &mut Gpio,
	running: &mut bool,
) -> std::io::Result<()> {
	let mut pin_l_f = gpio.get(L_F).unwrap().into_output();
	let mut pin_l_b = gpio.get(L_B).unwrap().into_output();
	let mut pin_r_f = gpio.get(R_F).unwrap().into_output();
	let mut pin_r_b = gpio.get(R_B).unwrap().into_output();
	pin_l_f.set_reset_on_drop(true);
	pin_l_b.set_reset_on_drop(true);
	pin_r_f.set_reset_on_drop(true);
	pin_r_b.set_reset_on_drop(true);
	while *running {
		let buf = &mut [0u8; 2];
		stream.read(buf)?;
		let header = buf[0];
		let value = magic_fn(buf[1]);
		println!("Got: {:?}", buf);
		match header {
			STOP => *running = false,
			LEFT => {
				handle_motor(&mut pin_l_f, &mut pin_l_b, value).unwrap();
			}
			RIGHT => {
				handle_motor(&mut pin_r_f, &mut pin_r_b, value).unwrap();
			}
			h => {
				eprintln!("Unexpected header from server: {} ", h)
			}
		}
	}
	Ok(())
}

fn handle_stream_differential() {
	//TODO write car with servo steering, and differential speed control per motor
}

use rppal::gpio;

const FREQUENCY: f64 = 100.0;

fn handle_motor(pin_f: &mut OutputPin, pin_b: &mut OutputPin, speed: f64) -> gpio::Result<()> {
	println!("Speed: {}", speed);
	if speed == 0.0 {
		pin_f.clear_pwm()?;
		pin_b.clear_pwm()?;
	} else if speed > 0.0 {
		pin_b.clear_pwm()?;
		pin_f.set_pwm_frequency(FREQUENCY, speed)?;
	} else if speed < 0.0 {
		pin_f.clear_pwm()?;
		pin_b.set_pwm_frequency(FREQUENCY, -speed)?;
	}
	Ok(())
}

const MIN_DUTY: f64 = 0.22;
const ROUND: i8 = 10;

fn magic_fn(num: u8) -> f64 {
	let num = unsafe { *(&num as *const u8 as *const i8) };
	if num < ROUND && num > -ROUND {
		return 0.0;
	}
	let num = num as f64;
	if num == 0.0 {
		return 0.0;
	}
	(num / std::i8::MAX as f64) * (1.0 - MIN_DUTY) + (MIN_DUTY * num.signum())
}
