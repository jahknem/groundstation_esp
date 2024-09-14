import serial
import time
import struct

def calculate_checksum(data):
    """Calculate checksum as XOR of all bytes in data."""
    checksum = 0
    for b in data:
        checksum ^= b
    return checksum

def construct_command(command_id, payload):
    """Construct command according to the protocol."""
    START_BYTE = 0x02
    END_BYTE = 0x03
    payload_length = len(payload)
    header = bytes([START_BYTE, command_id, payload_length])
    checksum = calculate_checksum(header + payload)
    command = header + payload + bytes([checksum, END_BYTE])
    return command

def send_command(ser, command_name, command_data):
    """Send command and print the response."""
    ser.write(command_data)
    print(f"Sent {command_name}: {command_data.hex()}")

    # Wait for response
    time.sleep(0.1)
    response = ser.read_all()
    if response:
        print(f"Received response: {response.hex()}")

def main():
    # Open the serial port (adjust 'port' as needed)
    ser = serial.Serial(
        port='/dev/serial0',  # Replace with your serial port
        baudrate=115200,
        timeout=1
    )

    try:
        while True:
            # Command 0x01: Move to Specific Position
            azimuth = 90    # Example azimuth in degrees
            elevation = 45  # Example elevation in degrees
            payload = struct.pack('>HH', azimuth, elevation)  # Big-endian unsigned shorts
            cmd = construct_command(0x01, payload)
            send_command(ser, "Move to Specific Position", cmd)
            time.sleep(0.5)

            # Command 0x02: Move by Increment
            delta_azimuth = -10  # Example delta azimuth
            delta_elevation = 5   # Example delta elevation
            payload = struct.pack('>hh', delta_azimuth, delta_elevation)  # Big-endian signed shorts
            cmd = construct_command(0x02, payload)
            send_command(ser, "Move by Increment", cmd)
            time.sleep(0.5)

            # Command 0x03: Read Current Position
            payload = b''  # No payload
            cmd = construct_command(0x03, payload)
            send_command(ser, "Read Current Position", cmd)
            time.sleep(0.5)

            # Command 0x04: Set Speed
            speed = 500  # Example speed value
            payload = struct.pack('>H', speed)  # Big-endian unsigned short
            cmd = construct_command(0x04, payload)
            send_command(ser, "Set Speed", cmd)
            time.sleep(0.5)

            # Command 0x05: Stop Motion
            payload = b''  # No payload
            cmd = construct_command(0x05, payload)
            send_command(ser, "Stop Motion", cmd)
            print("Sent Stop Motion")
            time.sleep(0.5)

            # Command 0x06: Set Mode
            mode = 0x00  # Example mode (e.g., Manual Mode)
            payload = bytes([mode])
            cmd = construct_command(0x06, payload)
            send_command(ser, "Set Mode", cmd)
            time.sleep(0.5)

            # Command 0x07: Request Status
            payload = b''  # No payload
            cmd = construct_command(0x07, payload)
            send_command(ser, "Request Status", cmd)
            time.sleep(0.5)

    except KeyboardInterrupt:
        print("Exiting...")
    finally:
        ser.close()

if __name__ == "__main__":
    main()
