// lib.rs
pub mod hall_sensor;
pub mod motor;
pub mod motor_controller;
pub mod uart;

pub mod command {
    include!(concat!(env!("OUT_DIR"), "/esp_turret.command.rs"));
}

