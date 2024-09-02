use accel_stepper::{Driver, OperatingSystemClock};
use esp_idf_hal::{peripheral::Peripheral, prelude::Peripherals, gpio::PinDriver, ledc::{LedcTimerDriver, config::TimerConfig, LedcDriver}};

mod stepper;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");


    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();

    let servo_timer = peripherals.ledc.timer1;
    let servo_driver = LedcTimerDriver::new(servo_timer, &TimerConfig::new().frequency(50.Hz()).resolution(esp_idf_hal::ledc::Resolution::Bits14)).unwrap();
    let servo = Arc::new(Mutex::new(LedcDriver::new(peripherals.ledc.channel3, servo_driver, peripherals.pins.gpio2).unwrap()));

    let mut driver = Driver::default();
    driver.set_max_speed(500.0);
    driver.set_acceleration(200.0);

    let driver = Arc::new(Mutex::new(driver));
    let endpoint_driver = driver.clone();
    let clock = OperatingSystemClock::new();

    let mut stepper = stepper::Stepper::new(peripherals.pins.gpio3, peripherals.pins.gpio4, peripherals.pins.gpio5, peripherals.pins.gpio6);
    // 2^14 - 1 

    let max_duty = servo.lock().unwrap().get_max_duty();

    let min = max_duty / 40;
    let max = max_duty / 8;

    fn interpolate(angle: u32, min: u32, max: u32)->u32 {
        angle * (max - min) / 180 + min
    }
 

    loop {
        driver.lock().unwrap().poll(&mut stepper, &clock).unwrap();
        sleep(Duration::from_micros(2000));
    }
    // keep polling the axis until it reaches that location
    while axis.is_running() {
        axis.poll(&mut dev, &clock)?;
    }
    log::info!("Reached dest")
}
