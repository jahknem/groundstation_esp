//main.rs

use esp_idf_sys::{self as _, EspError};
use esp_idf_hal::{gpio, peripherals::Peripherals};
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::{self, oneshot::*};
use esp_idf_hal::prelude::*;
use esp_idf_hal::task::*;
use esp_idf_hal::uart::{config, UartDriver, AsyncUartDriver};
use std::borrow::BorrowMut;
use std::thread;
use std::time::Duration;
use prost::Message;
use accel_stepper::{Driver, OperatingSystemClock};

extern crate esp_turret;

use esp_turret::uart::process_uart_data;
use esp_turret::motor_controller::MotorController;
use esp_turret::hall_sensor::calculate_degrees;

const GEAR_RATIO: f64 = 1.0 / 5.0;



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
    log::debug!("Debug message");

    let peripherals = Peripherals::take().unwrap();

    // UART Stuff
    let config = config::Config::default().baudrate(Hertz(115_200));
    let mut uart_driver = AsyncUartDriver::new(
        peripherals.uart2,
        peripherals.pins.gpio17,
        peripherals.pins.gpio16,
        Option::<gpio::AnyIOPin>::None,
        Option::<gpio::AnyIOPin>::None,
        &config
    ).unwrap();

    std::thread::spawn(move || {
        block_on(async {
            process_uart_data(&mut uart_driver).await;
        });
    });

    // ADC Stuff
    let adc = AdcDriver::new(peripherals.adc1).unwrap();
    let config = AdcChannelConfig {
        attenuation: DB_11,
        resolution: adc::oneshot::config::Resolution::Resolution12Bit,
        calibration: true,
        ..Default::default()
    };


    let mut adc_elevation = AdcChannelDriver::new(&adc, peripherals.pins.gpio34, &config).unwrap();
    let mut adc_azimuth = AdcChannelDriver::new(&adc, peripherals.pins.gpio35, &config).unwrap();

    // Stepper Stuff

    let azimuth_motor_pin = peripherals.pins.gpio32.into();
    let azimuth_direction_pin = peripherals.pins.gpio33.into();
    let elevation_motor_pin = peripherals.pins.gpio25.into();
    let elevation_direction_pin = peripherals.pins.gpio26.into();

    let mut motor_controller = MotorController::new(
        azimuth_motor_pin,
        azimuth_direction_pin,
        Some(GEAR_RATIO),
        elevation_motor_pin,
        elevation_direction_pin,
        Some(GEAR_RATIO),
    ).unwrap();


    loop {
        // Get the current sensor reading
        // motor_controller.turn_from_manual_control(0.0, 0.0);
        // log::info!(
        //     "Azimuth {}", 
        //     calculate_degrees(
        //         adc_min, 
        //         adc_max, 
        //         adc_azimuth.read().unwrap(), 
        //         adc_reference
        //     )
        // );
        // log::info!(
        //     "Elevation {}", 
        //     calculate_degrees(
        //         adc_min, 
        //         adc_max, 
        //         adc_elevation.read().unwrap(), 
        //         adc_reference
        //     )
        // );
        // motor_controller.turn_from_manual_control(30.0, 30.0);
        log::info!(
            "Azimuth {}", 
            calculate_degrees(
                adc_min, 
                adc_max, 
                adc_azimuth.read().unwrap(), 
                adc_reference
            )
        );
        log::info!(
            "Elevation {}", 
            calculate_degrees(
                adc_min, 
                adc_max, 
                adc_elevation.read().unwrap(), 
                adc_reference
            )
        );
        thread::sleep(Duration::from_millis(1000));
    }
}
