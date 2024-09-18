# Setup

1. Install rust 
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. Install espup (https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)
    ```
    cargo install espup
    cargo install cargo-generate
    cargo install cargo-espflash --locked
    cargo install espflash

3. Install necessary toolchains: 
    ```
    espup install -v 1.74.0.1
4. Install the prerequisites for std Dev: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#for-linux-users
    
    1. Install prerequisite packages to compile ESP-IDF: 
        ```
        sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
    2. Clone ESP IDF
        ```
        mkdir -p ~/esp
        cd ~/esp
        git clone --recursive https://github.com/espressif/esp-idf.git
    3. Install ESP-IDF Tools
        ```
        cd ~/esp/esp-idf
        ./install.sh all
    4. Export env Vars. (Do this every time you open this project!)
        ```
        . $HOME/esp/esp-idf/export.sh
5. Install ldproxy for proxying linker args
    ```
    cargo install ldproxy
6. Install espflash
    1. Install libudev and libc6
        ```
        apt-get install libudev-dev libc6
    2. Install espflash itself
        ```
        cargo install cargo-espflash --locked


# Antenna Gimbal Control System

## Introduction

This project involves developing software for an antenna gimbal/turret control system using an ESP32 microcontroller. The system measures the angular position of the antenna using Hall effect sensors connected to analog-to-digital converters (ADCs), controls stepper motors for precise positioning, and communicates with a Raspberry Pi via UART. The inclusion of JTAG allows for remote debugging of the ESP32 from the Raspberry Pi, which is crucial for maintenance when on-site access is limited.

## Requirements

### Hardware

- **ESP32 Development Board**
  - Supports ADC, UART, GPIO, and JTAG interfaces.
- **Stepper Motors (2x)**
  - For azimuth and elevation control.
- **Stepper Motor Drivers (2x)**
  - Accepts direction and pulse signals.
- **Hall Effect Sensors (2x)**
  - Provides angular position feedback.
- **Raspberry Pi**
  - Acts as a higher-level controller and debugging interface.
- **Adapters and Connectors**
  - 3-pin, 4-pin, and 5-pin adapters for sensors, drivers, UART, and JTAG connections.
- **Cabling and Power Supplies**
  - Appropriate cables and power supplies for all components.

### Software

- **ESP-IDF Framework**
  - For ESP32 development.
- **Rust Programming Language**
  - Main language for firmware development.
- **Python 3**
  - For UART testing scripts (`uart_test.py`).
- **Required Rust Crates**
  - `esp-idf-svc`, `esp-idf-hal`, `esp-idf-sys`, `accel-stepper`, etc.
- **Development Tools**
  - Rust toolchain, cargo, and embuild.
- **Debugging Tools**
  - JTAG debugger compatible with ESP32.

## Hardware Description and Pinout

### ESP32 Pinout Configuration

| GPIO  | Function          | Description                                  |
|-------|-------------------|----------------------------------------------|
| GPIO34| Hall Sensor 1     | Analog input for azimuth position feedback.  |
| GPIO35| Hall Sensor 2     | Analog input for elevation position feedback.|
| GPIO32| Stepper 1 Dir     | Direction control for azimuth motor.         |
| GPIO33| Stepper 1 Pulse   | Pulse control for azimuth motor steps.       |
| GPIO25| Stepper 2 Dir     | Direction control for elevation motor.       |
| GPIO26| Stepper 2 Pulse   | Pulse control for elevation motor steps.     |
| GPIO12| TDI (JTAG)        | JTAG interface for debugging.                |
| GPIO13| TCK (JTAG)        | JTAG clock signal.                           |
| GPIO14| TMS (JTAG)        | JTAG mode select.                            |
| GPIO15| TDO (JTAG)        | JTAG data output.                            |
| GPIO17| UART TX           | Transmit data to Raspberry Pi.               |
| GPIO16| UART RX           | Receive data from Raspberry Pi.              |

### Connectors and Adapters

- **Hall Sensor Adapters (2x 3-pin)**
  - VCC, Signal (GPIO34/GPIO35), GND.
- **Stepper Driver Adapters (2x 4-pin)**
  - Dir, GND, Pulse, GND.
- **JTAG Adapter (1x 5-pin)**
  - TDI, TCK, TMS, TDO, GND.
- **UART Adapter (1x 3-pin)**
  - TX (GPIO17), RX (GPIO16), GND.

### JTAG Interface

The JTAG interface is crucial for remote debugging from the Raspberry Pi. This eliminates the need to physically access the ESP32 for troubleshooting, which is particularly beneficial when on-site presence is not possible.

## Goals and Work Packages

The project is divided into several work packages, each containing specific tasks ranging from hardware interfacing to software functionality.

### Milestone 1: Hardware Setup and Verification

#### WP1.1: ESP32 Configuration

- **Objective**: Configure the ESP32 development board with the required pin assignments.
- **Tasks**:
  - Assign GPIOs as per the pinout.
  - Verify ADC channels for Hall sensors.
  - Configure UART communication parameters.

#### WP1.2: Hardware Connectivity

- **Objective**: Connect all hardware components according to the schematic.
- **Tasks**:
  - Connect Hall sensors to GPIO34 and GPIO35.
  - Connect stepper motor drivers to GPIO32/33 and GPIO25/26.
  - Set up JTAG interface for debugging.
  - Establish UART connection with Raspberry Pi.

### Milestone 2: Sensor Integration

#### WP2.1: ADC Reading Implementation

- **Objective**: Read analog values from Hall effect sensors.
- **Tasks**:
  - Initialize ADC channels with appropriate attenuation.
  - Implement continuous reading of ADC values.
  - Calibrate ADC readings for accurate angle measurements.

#### WP2.2: Angle Calculation Function

- **Objective**: Convert ADC values to angular positions.
- **Tasks**:
  - Implement `calculate_degrees` function.
  - Handle wrapping and scaling of ADC values.
  - Test and validate angle calculations against known positions.

### Milestone 3: Motor Control

#### WP3.1: Stepper Motor Driver Integration

- **Objective**: Interface with stepper motor drivers.
- **Tasks**:
  - Configure GPIOs for pulse and direction signals.
  - Implement low-level functions to send pulses to motors.
  - Ensure timing requirements are met for motor control signals.

#### WP3.2: High-Level Motor Control Functions

- **Objective**: Develop functions to move motors to specific positions.
- **Tasks**:
  - Implement functions to move motors to absolute angles.
  - Implement functions to move motors by relative increments.
  - Incorporate acceleration and deceleration profiles using `accel-stepper` crate.

#### WP3.3: Motor Control Testing

- **Objective**: Validate motor control functions.
- **Tasks**:
  - Test motor movements with various angles and increments.
  - Verify synchronization between sensor readings and motor positions.
  - Debug and optimize motor control algorithms.

### Milestone 4: Communication Protocol

#### WP4.1: UART Communication Setup

- **Objective**: Establish UART communication with Raspberry Pi.
- **Tasks**:
  - Configure UART parameters (baud rate, data bits, etc.).
  - Implement UART read and write functions.
  - Handle buffering and parsing of incoming data.

#### WP4.2: Command Parsing and Processing

- **Objective**: Develop a protocol for commands received via UART.
- **Tasks**:
  - Define start and end bytes (e.g., `0x02` and `0x03`).
  - Implement `parse_message` function to extract complete messages.
  - Process commands based on command IDs and payloads.

#### WP4.3: Response Generation

- **Objective**: Send appropriate responses back to the Raspberry Pi.
- **Tasks**:
  - Implement checksum calculation and verification.
  - Formulate response messages according to the protocol.
  - Handle error conditions and send error codes when necessary.

### Milestone 5: Software Integration and Testing

#### WP5.1: Integration Testing

- **Objective**: Test the integration of sensor readings, motor control, and communication.
- **Tasks**:
  - Run end-to-end tests using `uart_test.py`.
  - Validate that commands from the Raspberry Pi result in correct motor movements.
  - Ensure that sensor feedback is accurately reported.

#### WP5.2: Debugging and Optimization

- **Objective**: Use JTAG for debugging and optimize the system.
- **Tasks**:
  - Identify and fix any software bugs.
  - Optimize code for performance and reliability.
  - Ensure that the system can be remotely debugged via JTAG.

### Milestone 6: Additional Features and Improvements

#### WP6.1: Implement Safety Mechanisms

- **Objective**: Ensure safe operation of the gimbal system.
- **Tasks**:
  - Implement limit switches or software limits to prevent over-rotation.
  - Incorporate emergency stop functionality.
  - Add overcurrent and overheating protections.

#### WP6.2: Mode Selection and Configuration

- **Objective**: Allow the system to operate in different modes.
- **Tasks**:
  - Implement manual and automatic control modes.
  - Provide commands to switch modes via UART.
  - Ensure that mode changes are handled gracefully.

#### WP6.3: Documentation and User Guide

- **Objective**: Provide comprehensive documentation.
- **Tasks**:
  - Document all functions and modules in the code.
  - Write a user guide for operating the system.
  - Include troubleshooting steps and FAQs.

### Milestone 7: Deployment and Maintenance

#### WP7.1: Remote Update Capability

- **Objective**: Enable remote firmware updates.
- **Tasks**:
  - Implement a bootloader that supports OTA updates.
  - Secure the update process to prevent unauthorized access.
  - Test the update mechanism thoroughly.

#### WP7.2: Long-Term Maintenance Plan

- **Objective**: Establish a plan for maintaining the system.
- **Tasks**:
  - Set up monitoring to detect and report issues.
  - Schedule regular maintenance checks.
  - Provide training materials for on-site personnel.

## UART Protocol Description

The communication between the ESP32 and the Raspberry Pi is established via UART using a custom protocol. This section details the protocol structure, command IDs, payloads, data types, and the functions required to handle them.

#### Protocol Overview

- **Half-Duplex Communication**: Data transmission can occur in both directions but not simultaneously.
- **Message ID**: Each message includes a unique `MESSAGE_ID`, allowing the sender to track acknowledgments and match responses to requests.
- **Acknowledgment Mechanism**: Every packet sent must be acknowledged by the receiver.
- **Error Handling**: Packets with invalid checksums are ignored.
- **Retransmission Strategy**: If a packet is not acknowledged within a certain timeout, it is retransmitted with an exponential backoff, up to a maximum backoff of 1 second.


#### Message Format

Each message follows the structure:

| Field           | Size (Bytes) | Description                                       |
|-----------------|--------------|---------------------------------------------------|
| **START_BYTE**  | 1            | Start of message indicator (`0x02`)               |
| **MESSAGE_ID**  | 1            | Unique random identifier for the message (0-255)  |
| **COMMAND_ID**  | 1            | Identifier for the command                        |
| **PAYLOAD_LEN** | 1            | Length of the payload in bytes                    |
| **PAYLOAD**     | Variable     | Command-specific data                             |
| **CHECKSUM**    | 1            | XOR of bytes from MESSAGE_ID to last PAYLOAD byte |
| **END_BYTE**    | 1            | End of message indicator (`0x03`)                 |


#### Acknowledgment Mechanism

- **ACK Message**: An acknowledgment is sent in response to each received message.
- **ACK Format**:

  | Field           | Size (Bytes) | Description                                         |
  |-----------------|--------------|-----------------------------------------------------|
  | **START_BYTE**  | 1            | `0x02`                                              |
  | **MESSAGE_ID**  | 1            | `MESSAGE_ID` of the message being acknowledged plus 1 |
  | **COMMAND_ID**  | 1            | `0x06` (ACK Command ID)                             |
  | **PAYLOAD_LEN** | 1            | `0x01`                                              |
  | **PAYLOAD**     | 1            |                                                     |
  | **CHECKSUM**    | 1            | XOR of COMMAND_ID, PAYLOAD_LEN, and PAYLOAD         |
  | **END_BYTE**    | 1            | `0x03`                                              |

- **ACK Timing**: The ACK must be sent immediately upon receiving a valid message.
- **Retransmission**: If an ACK is not received within a timeout period, the sender retransmits the message with an exponential backoff, up to a maximum backoff of 1 second.


### Commands and Their IDs

| COMMAND_ID | Description                | Payload                                               |
|------------|----------------------------|-------------------------------------------------------|
| `0x01`     | Move to Specific Position  | 4 bytes: [Azimuth (2 bytes), Elevation (2 bytes)]     |
| `0x02`     | Move by Increment          | 4 bytes: [Delta Azimuth (2 bytes), Delta Elevation (2 bytes)] |
| `0x03`     | Read Current Position      | None                                                  |
| `0x04`     | Set Speed                  | 2 bytes: [Speed (2 bytes)]                            |
| `0x05`     | Stop Motion                | None                                                  |
| `0x06`     | Set Mode                   | 1 byte: [Mode]                                        |
| `0x07`     | ACK (Acknowledgment)       | None                                                   |
| `0x08`     | Set Mode                   | 1 byte: [Mode]                                         |
| `0x09`     | Request Status             | None                                                   |

#### Checksum Calculation

The checksum is calculated as the XOR of all bytes from `MESSAGE_ID` to the last byte of the `PAYLOAD`. The `START_BYTE` and `END_BYTE` are excluded from the checksum calculation.

#### Error Handling

- **Invalid Checksum**: Messages with an invalid checksum are ignored.
- **Timeouts**: If an ACK is not received within the timeout period, the sender retransmits the message.
- **Maximum Retransmissions**: Implement an exponential backoff strategy with a maximum backoff of 1 second.

#### Message IDs and Acknowledgments

- **Message ID Assignment**: The sender assigns a `MESSAGE_ID` to each message. It can be a random number between 0 and 255.
- **ACK Message ID**: The receiver acknowledges by sending back the `MESSAGE_ID` incremented by 1 (modulo 256).
- **Message ID Matching**: This mechanism allows the sender to match ACKs to the messages sent.

### Data Types

- **Azimuth/Elevation**: 16-bit unsigned integers (Big-endian)
- **Delta Azimuth/Elevation**: 16-bit signed integers (Big-endian)
- **Speed**: 16-bit unsigned integer (Big-endian)
- **Mode**: 8-bit unsigned integer

### Functions and Message Handling

Each command received via UART triggers specific functions to handle the requested operation.

#### Function: `parse_message(buffer: &[u8]) -> Option<(Vec<u8>, usize)>`

- **Description**: Parses incoming UART data to extract complete messages based on START_BYTE and END_BYTE.
- **Parameters**:
  - `buffer`: Reference to a byte slice containing UART data.
- **Returns**:
  - `Option<(Vec<u8>, usize)>`: A tuple containing the complete message and the number of bytes consumed.

#### Function: `process_uart_data(uart_buffer: &mut Vec<u8>)`

- **Description**: Processes the UART buffer to handle complete messages.
- **Parameters**:
  - `uart_buffer`: Mutable reference to the UART data buffer.
- **Operations**:
  - Calls `parse_message` to extract messages.
  - Validates and processes each message based on COMMAND_ID.

#### Command Handling Functions

Each command triggers specific functions to execute the desired operation.

##### 1. Move to Specific Position (`COMMAND_ID = 0x01`)

- **Payload**:
  - `Azimuth`: 2 bytes (unsigned 16-bit integer)
  - `Elevation`: 2 bytes (unsigned 16-bit integer)
- **Function**: `move_to_position(azimuth: u16, elevation: u16)`
- **Description**:
  - Moves the antenna to the specified azimuth and elevation angles.
- **Response**:
  - Acknowledgment message including current position and speed.

##### 2. Move by Increment (`COMMAND_ID = 0x02`)

- **Payload**:
  - `Delta Azimuth`: 2 bytes (signed 16-bit integer)
  - `Delta Elevation`: 2 bytes (signed 16-bit integer)
- **Function**: `move_by_increment(delta_azimuth: i16, delta_elevation: i16)`
- **Description**:
  - Adjusts the antenna position by the specified increments.
- **Response**:
  - Acknowledgment message including current position and speed.

##### 3. Read Current Position (`COMMAND_ID = 0x03`)

- **Payload**: None
- **Function**: `read_current_position() -> (u16, u16)`
- **Description**:
  - Retrieves the current azimuth and elevation angles.
- **Response**:
  - Payload containing current azimuth and elevation angles.

##### 4. Set Speed (`COMMAND_ID = 0x04`)

- **Payload**:
  - `Speed`: 2 bytes (unsigned 16-bit integer)
- **Function**: `set_speed(speed: u16)`
- **Description**:
  - Sets the movement speed of the motors.
- **Response**:
  - Acknowledgment message including current position and speed.

##### 5. Stop Motion (`COMMAND_ID = 0x05`)

- **Payload**: None
- **Function**: `stop_motion()`
- **Description**:
  - Immediately stops all motor movements.
- **Response**:
  - Acknowledgment message including current position and speed.

##### 6. Set Mode (`COMMAND_ID = 0x06`)

- **Payload**:
  - `Mode`: 1 byte (unsigned 8-bit integer)
- **Function**: `set_mode(mode: u8)`
- **Description**:
  - Sets the operation mode of the system (e.g., armed, disarmed).
- **Response**:
  - Acknowledgment message including current mode.

##### 7. Request Status (`COMMAND_ID = 0x07`)

- **Payload**: None
- **Function**: `request_status() -> Status`
- **Description**:
  - Retrieves the current status of the system.
- **Response**:
  - Payload containing current mode, position, speed and other data.

### Example Messages

##### Example: Acknowledging a Command

- **Received MESSAGE_ID**: `0x5A`
- **ACK MESSAGE_ID**: `0x5B` (`0x5A` + 1 modulo 256)

- **ACK Message**:

  | Field           | Value                                   | Hex Representation      |
  |-----------------|-----------------------------------------|-------------------------|
  | **START_BYTE**  | `0x02`                                  | `0x02`                  |
  | **MESSAGE_ID**  | `0x5B`                                  | `0x5B`                  |
  | **COMMAND_ID**  | `0x06` (ACK Command)                    | `0x06`                  |
  | **PAYLOAD_LEN** | `0x00`                                  | `0x00`                  |
  | **CHECKSUM**    | `0x5B ^ 0x06 ^ 0x00` = `0x5D`           | `0x5D`                  |
  | **END_BYTE**    | `0x03`                                  | `0x03`                  |

##### Example: Responding to a Read Request

- **Command**: Read Current Position (`COMMAND_ID = 0x03`)
- **MESSAGE_ID**: `0xA4` (randomly chosen)
- **Payload**: None

- **Request Message**:

  | Field           | Value                             | Hex Representation |
  |-----------------|-----------------------------------|--------------------|
  | **START_BYTE**  | `0x02`                            | `0x02`             |
  | **MESSAGE_ID**  | `0xA4`                            | `0xA4`             |
  | **COMMAND_ID**  | `0x03`                            | `0x03`             |
  | **PAYLOAD_LEN** | `0x00`                            | `0x00`             |
  | **PAYLOAD**     | None                              |                    |
  | **CHECKSUM**    | `0xA4 ^ 0x03 ^ 0x00` = `0xA7`     | `0xA7`             |
  | **END_BYTE**    | `0x03`                            | `0x03`             |

- **ACK Message**:

  - **MESSAGE_ID**: `0xA5` (`0xA4` + 1)

- **Response Message**:

  - **MESSAGE_ID**: `0xA5` (same as ACK `MESSAGE_ID`)
  - **COMMAND_ID**: `0x09` (Response Message)
  - **PAYLOAD_LEN**: `0x04` (assuming 4 bytes for azimuth and elevation)
  - **PAYLOAD**: Current azimuth and elevation values
  - **CHECKSUM**: Calculated over `MESSAGE_ID`, `COMMAND_ID`, `PAYLOAD_LEN`, and `PAYLOAD`


## Implementation Status

- **Implemented**:
- ADC reading and angle calculation (`hall_sensor.rs`).
- Basic UART communication and message parsing.
- Initial motor control functions using `accel-stepper`.
- UART testing script (`uart_test.py`).

- **In Progress**:
- Full implementation of command processing.
- Integration of motor control with sensor feedback.
- Remote debugging setup via JTAG.

- **To Be Implemented**:
- Safety mechanisms and emergency stop.
- Mode selection features.
- Remote firmware update capability.
- Comprehensive documentation and user guides.

## Contributing

Contributions to enhance the functionality, fix bugs, or improve documentation are welcome. Please follow the standard Git workflow for submitting pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

