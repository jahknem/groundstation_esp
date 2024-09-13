use esp_idf_sys::{self as _};
use esp_idf_hal::{gpio, ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver}, peripherals::Peripherals};
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::*;
use esp_idf_hal::uart;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use accel_stepper::{Driver, OperatingSystemClock};

mod hall_sensor;
use hall_sensor::calculate_degrees;



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

    // UART Stuff

    let config = uart::config::Config::default().baudrate(Hertz(115_200));

    let mut uart_driver: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart0,
        peripherals.pins.gpio1,
        peripherals.pins.gpio3,
        Option::<gpio::AnyIOPin>::None,
        Option::<gpio::AnyIOPin>::None,
        &config
    ).unwrap();
    // // Main loop to access the sensor degrees periodically
    let uart_driver = Arc::new(Mutex::new(uart_driver));

    // Spawn a thread to read from UART
    {
        let uart_driver = Arc::clone(&uart_driver);
        thread::spawn(move || {
            loop {
                let mut buf = [0u8; 128]; // Buffer for reading data
                let mut uart = uart_driver.lock().unwrap();
                match uart.read(&mut buf, 1000) {
                    Ok(n) => {
                        if n > 0 {
                            let data = &buf[..n];
                            let s = String::from_utf8_lossy(data);
                            log::info!("Received data via UART: {}", s);

                            // Echo back the received data with "echo" prefix
                            let response = format!("echo{}", s);
                            if let Err(err) = uart.write(response.as_bytes()) {
                                log::error!("UART write error: {:?}", err);
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("UART read error: {:?}", err);
                    }
                }
            }
        });
    }
    // ADC Stuff
    let adc = AdcDriver::new(peripherals.adc1).unwrap();
    let config = AdcChannelConfig {
        attenuation: DB_11,
        resolution: config::Resolution::Resolution12Bit,
        calibration: true,
        ..Default::default()
    };


    let mut adc_elevation = AdcChannelDriver::new(&adc, peripherals.pins.gpio34, &config).unwrap();
    let mut adc_azimuth = AdcChannelDriver::new(&adc, peripherals.pins.gpio35, &config).unwrap();

    // // Stepper Stuff
    // let mut axis = Driver::new();
    // axis.set_max_speed(500.0);
    // axis.set_acceleration(100.0);

    // let mut forward = 0;
    // let mut back = 0;

    // let mut dev = accel_stepper::func_device(|| forward += 1, || back += 1);

    // axis.move_to(17);

    // let clock = OperatingSystemClock::new();
    // while axis.is_running() {
    //     axis.poll(&mut dev, &clock).unwrap();
    // }
    loop {
        // Get the current sensor reading
        thread::sleep(Duration::from_millis(5000));
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
    }
}
