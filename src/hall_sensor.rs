use esp_idf_hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::config::Resolution;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::adc::{Adc};
use esp_idf_hal::gpio::ADCPin;

use std::ops::DerefMut;
use std::sync::{Mutex, Arc};

use core::borrow::Borrow;

pub struct HallSensor<T> 
where
    T: ADCPin + Clone,
{
    adc: Arc<Mutex<AdcDriver<'static, T::Adc>>>,
    gpio_pin: T,
    config: AdcChannelConfig,
    cal_low_degrees: u16, 
    cal_low_sample: u16, 
    cal_high_degrees: u16, 
    cal_high_sample: u16, 
}

impl<T> HallSensor<T> 
where
    T: ADCPin + Clone,               // Ensure T is compatible with ADCPin
{
    pub fn new(adc: Arc<Mutex<AdcDriver<'static, T::Adc>>>, gpio_pin: T, config: AdcChannelConfig) -> Self
    {
        let res = match config.resolution {
            Resolution::Resolution9Bit => 2^9,    // 9-bit resolution -> max value 511
            Resolution::Resolution10Bit => 2^10,  // 10-bit resolution -> max value 1023
            Resolution::Resolution11Bit => 2^11,  // 11-bit resolution -> max value 2047
            Resolution::Resolution12Bit => 2^12,  // 12-bit resolution -> max value 4095
        };
        HallSensor {
            adc,
            gpio_pin: gpio_pin,
            config: config.clone(),
            cal_high_degrees: 360,
            cal_high_sample: res,
            cal_low_degrees: 0,
            cal_low_sample: 0,
        }
    }

    pub fn read_degrees(&mut self) -> u16 {
        let mut adc_guard = self.adc.lock().unwrap();
        let mut adc_channel_driver = AdcChannelDriver::new(adc_guard.deref_mut(), self.gpio_pin.clone(), &self.config).unwrap();
        self.cal_low_degrees + (self.cal_high_degrees - self.cal_low_degrees) * (adc_channel_driver.read().unwrap() - self.cal_low_sample) / (self.cal_high_sample - self.cal_low_sample)
    }
}

