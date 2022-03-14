#![allow(dead_code)]
pub fn servo() {
	use rppal::gpio;
	use std::time::Duration;
	let pins = gpio::Gpio::new().unwrap();
	let mut servo = pins.get(17).unwrap().into_output();
	servo.set_reset_on_drop(true);
	//From 0.5 millis to 2.5 millis
	for i in 0..180 {
		servo
			.set_pwm(
				Duration::from_millis(20),
				Duration::from_micros(500 + 2000 * i / 180),
			)
			.unwrap();
		println!("{}", 500 + 2500 * i / 180);
		if i == 0 {std::thread::sleep(Duration::from_millis(1000))}
		std::thread::sleep(Duration::from_millis(50));
	}
	std::thread::sleep_ms(1000);
}

pub fn min_volt_motor() {
	use rppal::gpio;
	let pins = gpio::Gpio::new().unwrap();
	let mut servo = pins.get(23).unwrap().into_output();
	servo.set_reset_on_drop(true);
	const MAX_TEST: u32 = 100;
	for i in 0..1000 {
		servo.set_pwm_frequency(
			100.0,
			(i as f64) / MAX_TEST as f64 
		).unwrap();
		println!("{i}");
		std::thread::sleep_ms(100000 / MAX_TEST);
	}
	// 22 (22%) at 5.4V = 1.2V

}
