use std::error::Error;
use esp_idf_hal::gpio::{AnyIOPin, Output, PinDriver};
use esp_idf_hal::rmt::{TxRmtDriver, FixedLengthSignal, Pulse, PulseTicks, PinState, duration_to_ticks};
use esp_idf_hal::rmt::config::{TransmitConfig, Loop};
use esp_idf_sys::EspError;
use std::time::Duration;

pub struct Motor<'d> {
    direction_pin: PinDriver<'d, AnyIOPin, Output>, // PinDriver for direction control
    rmt_driver: TxRmtDriver<'d>,                    // TxRmtDriver for pulse control
    gear_ratio: f32,                                // Gear ratio of the motor
}

impl<'d> Motor<'d> {
    /// Constructor for a new Motor object.
    ///
    /// * `direction_pin` - A `PinDriver` controlling the motor direction.
    /// * `rmt_driver` - A `TxRmtDriver` controlling the motor pulses.
    /// * `gear_ratio` - Optional ratio between motor's and moving arm's gears. Defaults to `1.0`.
    pub fn new(
        direction_pin: PinDriver<'d, AnyIOPin, Output>,
        rmt_driver: TxRmtDriver<'d>,
        gear_ratio: Option<f64>,
    ) -> Self {
        let gear_ratio = gear_ratio.unwrap_or(1.0) as f32;

        Motor {
            direction_pin,
            rmt_driver,
            gear_ratio,
        }
    }

    /// Sets the direction of the motor.
    ///
    /// * `direction` - `true` for clockwise, `false` for counter-clockwise.
    pub fn set_direction(&mut self, direction: bool) -> Result<(), EspError> {
        if direction {
            self.direction_pin.set_high()
        } else {
            self.direction_pin.set_low()
        }
    }

    /// Starts the motor at the given speed (frequency in Hz).
    ///
    /// * `frequency_hz` - Frequency of the pulses in Hertz.
    pub fn start(&mut self, frequency_hz: u64) -> Result<(), Box<dyn Error>> {
        let ticks_hz = self.rmt_driver.counter_clock()?;

        // Calculate the pulse duration for the given frequency
        let pulse_duration = Duration::from_micros(1_000_000 / (frequency_hz * 2)); // Half-period
        let pulse_ticks = duration_to_ticks(ticks_hz, &pulse_duration)?;

        // Create a signal with the updated pulse duration
        let mut signal = FixedLengthSignal::<1>::new();
        let high_pulse = Pulse::new(PinState::High, PulseTicks::new(pulse_ticks)?);
        let low_pulse = Pulse::new(PinState::Low, PulseTicks::new(pulse_ticks)?);
        signal.set(0, &(high_pulse, low_pulse))?;

        // Start the signal and set looping to endless
        self.rmt_driver.start(signal)?;
        self.rmt_driver.set_looping(Loop::Endless)?;

        Ok(())
    }

    /// Updates the motor speed without stopping the RMT driver.
    ///
    /// * `frequency_hz` - New frequency of the pulses in Hertz.
    pub fn update_speed(&mut self, frequency_hz: u64) -> Result<(), Box<dyn Error>> {
        let ticks_hz = self.rmt_driver.counter_clock()?;

        // Calculate the pulse duration for the new frequency
        let pulse_duration = Duration::from_micros(1_000_000 / (frequency_hz * 2)); // Half-period
        let pulse_ticks = duration_to_ticks(ticks_hz, &pulse_duration)?;

        // Create a new signal with the updated pulse duration
        let mut signal = FixedLengthSignal::<1>::new();
        let high_pulse = Pulse::new(PinState::High, PulseTicks::new(pulse_ticks)?);
        let low_pulse = Pulse::new(PinState::Low, PulseTicks::new(pulse_ticks)?);
        signal.set(0, &(high_pulse, low_pulse))?;

        // Start the new signal
        self.rmt_driver.start(signal)?;
        // Looping is already set to endless

        Ok(())
    }

    /// Stops the motor.
    pub fn stop(&mut self) -> Result<(), EspError> {
        self.rmt_driver.stop()
    }

    /// Returns the gear ratio.
    pub fn gear_ratio(&self) -> f32 {
        self.gear_ratio
    }
}
