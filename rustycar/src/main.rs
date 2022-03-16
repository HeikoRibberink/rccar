use rppal::gpio::Gpio;
use rppal::hal::Delay;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

// mod test;
mod motor;
use motor::{motor, motor_speed_to_duty};


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
			// eprintln!("{:?}", handle_stream_tank(stream, &mut gpio, &mut running).err());
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
const LEFT_X: u8 = 1;
const LEFT_Y: u8 = 2;
const RIGHT_X: u8 = 3;
const RIGHT_Y: u8 = 4;

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
	let mut servo = gpio.get(SERVO).unwrap().into_output();
	let mut speed: f64 = 0.0;
	let mut steering: f64 = 0.0;
	while *running {
		let buf = &mut [0u8; 2];
		stream.read(buf)?;
		let header = buf[0];
		let value = buf[1];
		match header {
			STOP => *running = false,
			LEFT_Y => {
				motor(&mut pin_l_f, &mut pin_l_b, motor_speed_to_duty(unsigned_to_signed(value) as f64)).unwrap();
			}
			RIGHT_Y => {
				motor(&mut pin_r_f, &mut pin_r_b, motor_speed_to_duty(unsigned_to_signed(value) as f64)).unwrap();
			}
			h => {
				eprintln!("Unexpected header from server: {h} with value {value}")
			}
		}
	}
	Ok(())
}

fn steer() ->  {

}

const delta: f64 = 0.00001;
const length: f64 = 5;
const width: f64 = 1;
fn differential_steering(a: f64) -> (f64, f64) {
	if a == 0 {return (1.0, 1.0)}
	inner = (delta * a.cos() / (delta * a.sin() / length)) - width * 0.5;
	outer = inner + width;
	inner /= outer;
	outer = 1.0; //outer /= outer
	if a > 0 {(inner, outer)} else {(outer, inner)}
}

fn unsigned_to_signed(num: u8) -> i8 {
	unsafe { *(&num as *const u8 as *const i8) }
}
