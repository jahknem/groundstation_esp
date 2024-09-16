// motor.rs

use std::error::Error;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{AnyIOPin, Output, PinDriver};

const PULSES_PER_REVOLUTION: u16 = 6400; // Adjust as per your hardware
const DEFAULT_SECONDS_PER_REVOLUTION: f64 = 3.0;

#[derive(PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

pub struct Motor<'d> {
    motor_pin: PinDriver<'d, AnyIOPin, Output>,
    direction_pin: PinDriver<'d, AnyIOPin, Output>,
    gear_ratio: f64,
    delay: Delay,
}

impl<'d> Motor<'d> {
    /// Constructor for new Motor object.
    ///
    /// * `motor_pin` - GPIO pin for the motor pulse.
    /// * `direction_pin` - GPIO pin for the motor direction.
    /// * `gear_ratio` - Ratio between motor's and moving arm's gears. If `None`, defaults to `1.0`.
    pub fn new(
        motor_pin: AnyIOPin,
        direction_pin: AnyIOPin,
        gear_ratio: Option<f64>,
    ) -> Result<Self, Box<dyn Error>> {
        let motor_pin = PinDriver::output(motor_pin)?;
        let direction_pin = PinDriver::output(direction_pin)?;
        let gear_ratio = gear_ratio.unwrap_or(1.0);
        let delay = Delay::default();
        Ok(Motor {
            motor_pin,
            direction_pin,
            gear_ratio,
            delay,
        })
    }

    /// Rotates the moving arm by the specified angle in radians.
    pub fn turn_radians(
        &mut self,
        angle: f64,
        seconds_per_revolution: Option<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let pulses = (PULSES_PER_REVOLUTION as f64
            / std::f64::consts::TAU
            * angle
            * self.gear_ratio.recip())
            .floor() as i64;

        let direction = if pulses.is_positive() {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };
        self.turn_pulses(
            pulses.abs(),
            direction,
            seconds_per_revolution.unwrap_or(DEFAULT_SECONDS_PER_REVOLUTION),
        )
    }

    /// Rotates the moving arm by the specified angle in degrees.
    pub fn turn_degrees(
        &mut self,
        angle: f64,
        seconds_per_revolution: Option<f64>,
    ) -> Result<(), Box<dyn Error>> {
        self.turn_radians(angle.to_radians(), seconds_per_revolution)
    }

    /// Rotates the motor by the specified number of pulses.
    fn turn_pulses(
        &mut self,
        pulses: i64,
        direction: Direction,
        seconds_per_revolution: f64,
    ) -> Result<(), Box<dyn Error>> {
        let waiting_time_in_seconds = (seconds_per_revolution / PULSES_PER_REVOLUTION as f64) / 2.0;
        let waiting_duration_us = (waiting_time_in_seconds * 1_000_000.0) as u32;

        // Set direction pin
        match direction {
            Direction::Clockwise => self.direction_pin.set_high()?,
            Direction::CounterClockwise => self.direction_pin.set_low()?,
        }

        for _ in 0..pulses {
            self.pulse(waiting_duration_us);
        }
        Ok(())
    }

    /// Pulses the motor once; `waiting_duration_us` is half of the total pulse period.
    fn pulse(&mut self, waiting_duration_us: u32) {
        self.motor_pin.set_high().unwrap();
        self.delay.delay_us(waiting_duration_us);
        self.motor_pin.set_low().unwrap();
        self.delay.delay_us(waiting_duration_us);
    }
}
