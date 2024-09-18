// uart.rs


use esp_idf_sys::{self as _, EspError};
use crate::command::Command;
use crate::command::command::CommandType;
use std::borrow::BorrowMut;
use prost::Message;

use esp_idf_hal::task::*;
use esp_idf_hal::uart::*;

pub async fn process_uart_data<'a, T>(
    uart_driver: &mut AsyncUartDriver<'a, T>
) -> Result<(), EspError> 
where 
    T: BorrowMut<UartDriver<'a>>,
{
    let mut uart_buffer = vec![0u8; 128];

    loop {
        let bytes_read = uart_driver.read(&mut uart_buffer).await?;
        if bytes_read > 0 {
            log::info!("Read {} bytes from UART", bytes_read);

            // Decode Protobuf message
            match Command::decode(&uart_buffer[..bytes_read]) {
                Ok(command) => {
                    match command.command_type.unwrap() {
                        CommandType::SetPosition(set_pos) => {
                            log::info!(
                                "Set Position Command: Azimuth = {}, Elevation = {}",
                                set_pos.azimuth, set_pos.elevation
                            );
                            // Call your motor controller to set position
                        }
                        CommandType::SetVelocity(set_vel) => {
                            log::info!(
                                "Set Velocity Command: Azimuth Velocity = {}, Elevation Velocity = {}",
                                set_vel.azimuth_velocity, set_vel.elevation_velocity
                            );
                            // Call your motor controller to set velocity
                        }
                        CommandType::GetPosition(_) => {
                            log::info!("Get Position Command received");
                            // Handle Get Position logic
                        }
                        CommandType::GetVelocity(_) => {
                            log::info!("Get Velocity Command received");
                            // Handle Get Velocity logic
                        }
                        CommandType::SetCalibration(set_cal) => {
                            log::info!(
                                "Set Calibration Command: Azimuth = {}, Elevation = {}",
                                set_cal.azimuth, set_cal.elevation
                            );
                            // Call motor controller to set calibration
                        }
                    }
                }
                Err(e) => log::error!("Failed to parse message: {:?}", e),
            }
        }
    }
}
