import serial
import time

# === Configuration ===
SERIAL_PORT = "COM9"  # Change this according to your system (e.g., "/dev/ttyUSB0" on Linux)
BAUD_RATE = 115200
TIMEOUT = 1  # Seconds

# === Command List ===
COMMANDS = [
    (0x01, [0x00]),   # Analog Read (All)
    (0x01, [0x02]),
    (0x02, [0x02]),   # Port Read (Port B)
    (0x03, [0x02, 255]),  # Port Write (Port B, Set All High)
    (0x04, [0x03, 255]),  # Port Direction (Port B, Set All Output)
    (0x05, [7]),      # Digital Read (Pin 7)
    (0x06, [7, 1]),   # Digital Write (Pin 7, HIGH)
    (0x07, [6, 128]), # PWM Write (Pin 6, 50% duty cycle)
    (0x08, [7, 1]),   # Pin Mode (Pin 7, OUTPUT)
]

# === Functions ===
def send_command(ser, command_id, params):
    """Send a command packet to Arduino"""
    packet = [0x02, command_id] + params + [0x03]  # STX, Command ID, Params, ETX
    ser.write(bytearray(packet))
    time.sleep(0.1)  # Small delay for response

def receive_response(ser):
    """Receive and process response from Arduino"""
    response = ser.read_all()
    if response:
        return list(response)
    return []

# === Main Execution ===
def main():
    try:
        with serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=TIMEOUT) as ser:
            time.sleep(2)  # Allow time for Arduino to initialize

            for command_id, params in COMMANDS:
                print(f"Sending command 0x{command_id:X} with params {params}")
                send_command(ser, command_id, params)
                
                response = receive_response(ser)
                print(f"Response: {response}\n")
    
    except serial.SerialException as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
