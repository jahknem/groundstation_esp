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
use log::LevelFilter;

mod hall_sensor;
use hall_sensor::calculate_degrees;

fn parse_message(buffer: &[u8]) -> Option<(Vec<u8>, usize)> {
    // Define your protocol's start and end bytes
    const START_BYTE: u8 = 0x02; // Example start byte
    const END_BYTE: u8 = 0x03;   // Example end byte

    // Look for the start byte
    if let Some(start_index) = buffer.iter().position(|&b| b == START_BYTE) {
        // Look for the end byte after the start byte
        if let Some(end_index) = buffer[start_index + 1..].iter().position(|&b| b == END_BYTE) {
            let end_index = start_index + 1 + end_index;
            // Extract the message including start and end bytes
            let message = buffer[start_index..=end_index].to_vec();
            let consumed_bytes = end_index + 1; // Number of bytes to remove from the buffer
            return Some((message, consumed_bytes));
        }
    }

    // No complete message found yet
    None
}


fn process_uart_data(uart_buffer: &mut Vec<u8>) {
    // For now, simply log the data
    // if !uart_buffer.is_empty() {
    //     // Log raw data
    //     log::info!("UART Buffer: {:?}", uart_buffer);

    //     // Optionally, convert to string if data is UTF-8 encoded
    //     let data_str = String::from_utf8_lossy(uart_buffer);
    //     log::info!("UART Data as String: {}", data_str);

    //     // Clear the buffer after processing
    //     uart_buffer.clear();
    // }
    while let Some((message, consumed_bytes)) = parse_message(uart_buffer) {
        // Log the complete message
        log::info!("Received complete message: {:?}", message);

        // Remove the processed bytes from the buffer
        uart_buffer.drain(..consumed_bytes);
    }
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
    log::debug!("Debug message");

    let peripherals = Peripherals::take().unwrap();

    // UART Stuff

    let config = uart::config::Config::default().baudrate(Hertz(115_200));

    let mut uart_driver: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart2,
        peripherals.pins.gpio17,
        peripherals.pins.gpio16,
        Option::<gpio::AnyIOPin>::None,
        Option::<gpio::AnyIOPin>::None,
        &config
    ).unwrap();
    // // Main loop to access the sensor degrees periodically
    let uart_driver = Arc::new(Mutex::new(uart_driver));

    // Spawn a thread to read from UART
    {
        let uart_driver = Arc::clone(&uart_driver);
        log::info!("Starting UART Thread...");
        thread::spawn(move || {
            let mut uart_buffer = Vec::new();
            loop {
                log::info!("Reading from UART...");
                let mut buf = [0u8; 128]; // Buffer for reading data

                // Lock uart driver to read data
                let mut uart = uart_driver.lock().unwrap();
                match uart.read(&mut buf, 100) {
                    Ok(n) => {
                        if n > 0 {
                            log::info!("Read {} bytes from UART", n);

                            // Append received data to the buffer
                            uart_buffer.extend_from_slice(&buf[..n]);
    
                            // Process the received data
                            process_uart_data(&mut uart_buffer);
                        }
                    },
                    Err(err) => {
                        log::error!("UART read error: {:?}", err);
                    }
                }
                // Release the UART lock
                drop(uart);

                // Sleep briefly to allow other threads to run
                thread::sleep(Duration::from_millis(10));
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
        thread::sleep(Duration::from_millis(15000));
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
