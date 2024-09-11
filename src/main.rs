use esp_idf_sys::{self as _};
use esp_idf_hal::{peripherals::Peripherals, ledc::{LedcTimerDriver, config::TimerConfig, LedcDriver}};
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::units::*;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use accel_stepper::{Driver, OperatingSystemClock};

// mod hall_sensor;
// use hall_sensor::HallSensor;

pub fn calculate_degrees(adc_min: u16, adc_max: u16, adc_value: u16, adc_reference: u16) -> f32 {
    let adc_span = adc_max - adc_min;
    let adc_cleaned_var = (adc_value - adc_reference + adc_span) % adc_span;
    adc_cleaned_var as f32 /(adc_span -1) as f32 *360 as f32
}

fn main() {
    let adc_min = 0;
    let adc_max = 4095;
    let adc_reference = 1000;
    use esp_idf_hal::adc::attenuation::DB_11;
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let adc = AdcDriver::new(peripherals.adc1).unwrap();
    let config = AdcChannelConfig {
        attenuation: DB_11,
        resolution: config::Resolution::Resolution12Bit,
        calibration: true,
        ..Default::default()
    };


    let mut adc_elevation = AdcChannelDriver::new(&adc, peripherals.pins.gpio34, &config).unwrap();
    let mut adc_azimuth = AdcChannelDriver::new(&adc, peripherals.pins.gpio35, &config).unwrap();

    let mut axis = Driver::new();
    axis.set_max_speed(500.0);
    axis.set_acceleration(100.0);

    let mut forward = 0;
    let mut back = 0;

    let mut dev = accel_stepper::func_device(|| forward += 1, || back += 1);

    axis.move_to(17);

    let clock = OperatingSystemClock::new();
    while axis.is_running() {
        axis.poll(&mut dev, &clock).unwrap();
    }
    // Main loop to access the sensor degrees periodically
    loop {
        // Get the current sensor reading
        thread::sleep(Duration::from_millis(100));
        log::info!("Azimuth {}", calculate_degrees(adc_min, adc_max, adc_azimuth.read().unwrap(), adc_reference));
        log::info!("Elevation {}", calculate_degrees(adc_min, adc_max, adc_elevation.read().unwrap(), adc_reference));
    }
}
