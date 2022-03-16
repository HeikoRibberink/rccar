use rppal::gpio::{self, OutputPin};
use std::time::Duration;

pub fn motor(pin_f: &mut OutputPin, pin_b: &mut OutputPin, speed: f64) -> gpio::Result<()> {
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
	const ROUND: f64 = 10.0;
	const MIN_DUTY: f64 = 0.22;
	if num < ROUND && num > -ROUND {
		return 0.0;
	}
	if num == 0.0 {
		return 0.0;
	}
	(num / std::i8::MAX as f64) * (1.0 - MIN_DUTY) + (MIN_DUTY * num.signum())
}

pub fn servo(pin: &mut OutputPin, angle: f64) -> gpio::Result<()> {
	use std::f64::consts::{PI, FRAC_1_PI};
	const MIN_ANGLE: f64 = -0.5 * PI;
	const MAX_ANGLE: f64 = 0.5 * PI;
	
	if angle < MIN_ANGLE || angle > MAX_ANGLE {
		panic!("Steering angle is outside of range [-PI/2; PI/2]");
	}
	pin.set_pwm(
		Duration::from_millis(20),
		Duration::from_micros((500.0 + 2000.0 * angle * FRAC_1_PI * 0.5) as u64),
	)?;
	Ok(())
}