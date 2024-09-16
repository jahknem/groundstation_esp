// motor_controller.rs

use crate::motor::Motor;
use esp_idf_hal::gpio::AnyIOPin;
use std::error::Error;

const MAX_SECONDS_PER_REVOLUTION: f64 = 10.0; // Adjust based on your hardware
const MIN_SECONDS_PER_REVOLUTION: f64 = 0.25; // Adjust based on your hardware

pub struct MotorController<'d> {
    motor_azimuth: Motor<'d>,
    motor_elevation: Motor<'d>,
}

impl<'d> MotorController<'d> {
    pub fn new(
        azimuth_motor_pin: AnyIOPin,
        azimuth_direction_pin: AnyIOPin,
        azimuth_gear_ratio: Option<f64>,
        elevation_motor_pin: AnyIOPin,
        elevation_direction_pin: AnyIOPin,
        elevation_gear_ratio: Option<f64>,
    ) -> Result<Self, Box<dyn Error>> {
        let motor_azimuth = Motor::new(
            azimuth_motor_pin,
            azimuth_direction_pin,
            azimuth_gear_ratio,
        )?;
        let motor_elevation = Motor::new(
            elevation_motor_pin,
            elevation_direction_pin,
            elevation_gear_ratio,
        )?;
        Ok(MotorController {
            motor_azimuth,
            motor_elevation,
        })
    }

    /// Used for steering the tracker manually (e.g., using a controller).
    pub fn turn_from_manual_control(&mut self, azimuth_input: f64, elevation_input: f64) {
        let azimuth_speed = calculate_speed_from_manual_control(azimuth_input);
        let elevation_speed = calculate_speed_from_manual_control(elevation_input);

        if azimuth_speed > 0.0 {
            let _ = self.motor_azimuth.turn_degrees(
                if azimuth_input.is_sign_positive() { 1.0 } else { -1.0 },
                Some(azimuth_speed),
            );
        }

        if elevation_speed > 0.0 {
            let _ = self.motor_elevation.turn_degrees(
                if elevation_input.is_sign_positive() { 1.0 } else { -1.0 },
                Some(elevation_speed),
            );
        }

        println!(
            "Turned motors manually with speed {:.2} azimuth and {:.2} elevation.",
            azimuth_speed, elevation_speed
        );
    }
}

/// Calculates speed based on manual control input.
fn calculate_speed_from_manual_control(input_value: f64) -> f64 {
    let input_value = input_value.clamp(-1.0, 1.0).abs();
    if input_value > 0.1 {
        let slope: f64 = (MAX_SECONDS_PER_REVOLUTION - MIN_SECONDS_PER_REVOLUTION) / 0.9;
        let y_intercept: f64 = MIN_SECONDS_PER_REVOLUTION - slope;
        slope * input_value + y_intercept
    } else {
        0.0
    }
}
