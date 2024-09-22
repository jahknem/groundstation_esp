// motor_controller.rs

use crate::motor::Motor;
use crate::hall_sensor::calculate_degrees;
use esp_idf_hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_hal::gpio::ADCPin;
use esp_idf_hal::units::FromValueType;
use esp_idf_sys::EspError;
use std::borrow::Borrow;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct MotorController<'d, A1, A2, M1, M2>
where
    A1: ADCPin,
    A2: ADCPin,
    M1: Borrow<AdcDriver<'d, A1::Adc>>,
    M2: Borrow<AdcDriver<'d, A2::Adc>>,
{
    motor_azimuth: Motor<'d>,
    motor_elevation: Motor<'d>,
    adc_azimuth: AdcChannelDriver<'d, A1, M1>,
    adc_elevation: AdcChannelDriver<'d, A2, M2>,
    adc_min: u16,
    adc_max: u16,
    adc_reference: u16,
}

impl<'d, A1, A2, M1, M2> MotorController<'d, A1, A2, M1, M2>
where
    A1: ADCPin,
    A2: ADCPin,
    M1: Borrow<AdcDriver<'d, A1::Adc>>,
    M2: Borrow<AdcDriver<'d, A2::Adc>>,
{
    pub fn new(
        motor_azimuth: Motor<'d>,
        motor_elevation: Motor<'d>,
        adc_azimuth: AdcChannelDriver<'d, A1, M1>,
        adc_elevation: AdcChannelDriver<'d, A2, M2>,
        adc_min: u16,
        adc_max: u16,
        adc_reference: u16,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            motor_azimuth,
            motor_elevation,
            adc_azimuth,
            adc_elevation,
            adc_min,
            adc_max,
            adc_reference,
        })
    }

    /// Moves the antenna pod to the desired azimuth and elevation angles.
    pub fn move_to_angles(&mut self, target_azimuth: f32, target_elevation: f32) -> Result<(), Box<dyn Error>> {
        // Read current positions from hall sensors
        let current_azimuth = calculate_degrees(
            self.adc_min,
            self.adc_max,
            self.adc_azimuth.read()?,
            self.adc_reference,
        );

        let current_elevation = calculate_degrees(
            self.adc_min,
            self.adc_max,
            self.adc_elevation.read()?,
            self.adc_reference,
        );

        // Calculate the error considering the gear ratio
        let azimuth_error = target_azimuth - (current_azimuth * self.motor_azimuth.gear_ratio());
        let elevation_error = target_elevation - (current_elevation * self.motor_elevation.gear_ratio());

        // Set directions based on error signs
        self.motor_azimuth.set_direction(azimuth_error > 0.0)?;
        self.motor_elevation.set_direction(elevation_error > 0.0)?;

        // Calculate speeds (you might want to implement a control algorithm here)
        let azimuth_speed = self.calculate_speed_from_error(azimuth_error);
        let elevation_speed = self.calculate_speed_from_error(elevation_error);

        // Start motors with calculated speeds
        if azimuth_speed > 0.0 {
            self.motor_azimuth.start(azimuth_speed as u64)?;
        } else {
            self.motor_azimuth.stop()?;
        }

        if elevation_speed > 0.0 {
            self.motor_elevation.start(elevation_speed as u64)?;
        } else {
            self.motor_elevation.stop()?;
        }

        Ok(())
    }

    /// Calculates speed based on the error between current and target positions.
    fn calculate_speed_from_error(&self, error: f32) -> f32 {
        let error_abs = error.abs();
        if error_abs > 1.0 {
            // Map error to speed (frequency)
            // For example, error of 1 degree corresponds to 100 Hz, 10 degrees to 1000 Hz
            // Adjust these values based on your hardware
            let min_speed = 100.0; // Hz
            let max_speed = 1000.0; // Hz
            let speed = min_speed + (max_speed - min_speed) * (error_abs / 10.0).min(1.0);
            speed
        } else {
            0.0
        }
    }

    /// Stops both motors.
    pub fn stop(&mut self) -> Result<(), EspError> {
        self.motor_azimuth.stop()?;
        self.motor_elevation.stop()
    }

    /// Control loop to continuously adjust the antenna position.
    pub fn control_loop(&mut self, target_azimuth: f32, target_elevation: f32) -> Result<(), Box<dyn Error>> {
        loop {
            self.move_to_angles(target_azimuth, target_elevation)?;
            thread::sleep(Duration::from_millis(100)); // Adjust as needed
        }
    }
}
