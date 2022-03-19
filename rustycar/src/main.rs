use rppal::gpio::Gpio;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

// mod test;
mod motor;
use motor::{motor, motor_speed_to_duty, servo};


fn main() {
	// test::servo();
	// test::min_volt_motor();
	car()
}

const ADDRESS: &str = "192.168.2.60:8080";

fn car() {
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

const L_F: u8 = 17;
const L_B: u8 = 27;
const R_F: u8 = 23;
const R_B: u8 = 24;
const SERVO: u8 = 25;

const STOP: u8 = 255;
// const LEFT_X: u8 = 0;
const LEFT_Y: u8 = 1;
const RIGHT_X: u8 = 2;
// const RIGHT_Y: u8 = 3;


fn handle_stream(
	mut stream: TcpStream,
	gpio: &mut Gpio,
	running: &mut bool,
) -> std::io::Result<()> {
	//TODO write car with servo steering, and differential speed control per motor
	//TODO rewrite differential formula
	let mut pin_l_f = gpio.get(L_F).unwrap().into_output();
	let mut pin_l_b = gpio.get(L_B).unwrap().into_output();
	let mut pin_r_f = gpio.get(R_F).unwrap().into_output();
	let mut pin_r_b = gpio.get(R_B).unwrap().into_output();
	let mut pin_servo = gpio.get(SERVO).unwrap().into_output();
	let mut speed: f64 = 0.0;
	let mut angle: f64 = 0.0;
	while *running {
		let buf = &mut [0u8; 2];
		stream.read(buf)?;
		let header = buf[0];
		let value = buf[1];
		match header {
			STOP => *running = false,
			LEFT_Y => {
				speed = byte_to_signed(value) as f64 / 128.0;
			}
			RIGHT_X => {
				angle = byte_to_signed(value) as f64 / 256.0 * std::f64::consts::PI;
			}
			h => {
				eprintln!("Unexpected header from server: {h} with value {value}")
			}
		}
		//Update motors
		let (left, right) = differential_steering(angle);
		println!("Angle: {} \t Speed: {} \t Steering: {}, {}", angle, speed, left * speed, right * speed);
		motor(&mut pin_l_f, &mut pin_l_b, motor_speed_to_duty(left * speed)).unwrap();
		motor(&mut pin_r_f, &mut pin_r_b, motor_speed_to_duty(right * speed)).unwrap();
		servo(&mut pin_servo, angle).unwrap();
	}
	Ok(())
}

const DELTA: f64 = 0.00001;
const LENGTH: f64 = 5.0;
const WIDTH: f64 = 1.0;
fn differential_steering(i: f64) -> (f64, f64) {
	let a = i.abs();
	if a == 0.0 {return (1.0, 1.0)}
	let mut inner = (DELTA * a.cos() / (DELTA * a.sin() / LENGTH).asin()) - WIDTH * 0.5;
	let mut outer = inner + WIDTH;
	inner /= outer;
	outer = 1.0; //outer /= outer
	if i > 0.0 {(inner, outer)} else {(outer, inner)}
}

fn byte_to_signed(num: u8) -> i8 {
	unsafe { *(&num as *const u8 as *const i8) }
}
