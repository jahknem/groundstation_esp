use esp_idf_sys::{self as _};
use esp_idf_hal::{peripherals::Peripherals, ledc::{LedcTimerDriver, config::TimerConfig, LedcDriver}};
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::units::*;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use accel_stepper::Driver;



// mod hall_sensor;
// use hall_sensor::HallSensor;

pub fn calculate_degrees(adc_min: u16, adc_max: u16, adc_value: u16, adc_reference: u16) -> f32 {
    let adc_span = adc_max - adc_min;
    let adc_cleaned_var = (adc_value - adc_reference + adc_span) % adc_span;
    adc_cleaned_var as f32 /(adc_span -1) as f32 *360 as f32
}

fn main() {
    let cal_degree_low = 0;
    let cal_degree_high = 359;
    let cal_value_low = 0;
    let cal_value_high = 4095;
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


    let mut adc_elevation_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio34, &config).unwrap();
    let mut adc_azimuth_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio35, &config).unwrap();

    let servo_timer = peripherals.ledc.timer1;
    let servo_driver = LedcTimerDriver::new(servo_timer, &TimerConfig::new().frequency(50.Hz()).resolution(esp_idf_hal::ledc::Resolution::Bits14)).unwrap();
    //let servo = Arc::new(Mutex::new(LedcDriver::new(peripherals.ledc.channel3, servo_driver, pin)));

    // Main loop to access the sensor degrees periodically
    loop {
        // Get the current sensor reading
        thread::sleep(Duration::from_millis(100));
        // log::info!("Azimuth {}", hall_sensor_azimuth.read_degrees());
        // log::info!("Elevation {}", hall_sensor_elevation.read_degrees());
    }
}
