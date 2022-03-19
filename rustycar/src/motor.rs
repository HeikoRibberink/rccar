use rppal::gpio::{self, OutputPin};
use std::time::Duration;

pub fn motor(pin_f: &mut OutputPin, pin_b: &mut OutputPin, speed: f64) -> gpio::Result<()> {
	if speed.abs() > 1.0 {
		panic!("Speed ({}) is outside of range [-1, 1]", speed)
	}
	const MOTOR_FREQUENCY: f64 = 100.0;
	if speed == 0.0 {
		pin_f.clear_pwm()?;
		pin_b.clear_pwm()?;
	} else if speed > 0.0 {
		pin_b.clear_pwm()?;
		pin_f.set_pwm_frequency(MOTOR_FREQUENCY, speed)?;
	} else if speed < 0.0 {
		pin_f.clear_pwm()?;
		pin_b.set_pwm_frequency(MOTOR_FREQUENCY, -speed)?;
	}
	Ok(())
}

pub fn motor_speed_to_duty(num: f64) -> f64 {
	const ROUND: f64 = 0.03; //Should be more than 0.
	const MIN_DUTY: f64 = 0.22;
	if num.abs() < ROUND {
		return 0.0;
	}
	num * (1.0 - MIN_DUTY) + (MIN_DUTY * num.signum()) // Lerp
}

pub fn servo(pin: &mut OutputPin, angle: f64) -> gpio::Result<()> {
	use std::f64::consts::{FRAC_2_PI, PI};
	//Angle from [-90*; 90*] = [-PI/2; PI/2]
	const MIN_ANGLE: f64 = -0.5 * PI;
	const MAX_ANGLE: f64 = 0.5 * PI;
	if angle < MIN_ANGLE || angle > MAX_ANGLE {
		panic!("Steering angle {} is outside of range [-PI/2; PI/2]", angle);
	}
	pin.set_pwm(
		Duration::from_millis(20),
		Duration::from_micros((1500.0 + 1000.0 * angle * FRAC_2_PI) as u64),
	)?;
	Ok(())
}
