#![allow(dead_code)]
use esp_idf_hal::uart::{UartDriver, config::Config, Uart};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::gpio::{OutputPin, InputPin, AnyIOPin};
use esp_idf_sys::esp_timer_get_time;
use anyhow::Error;

/// Constants for servo command codes (from LewanSoul LX-16A protocol)
const CMD_MOVE_TIME_WRITE: u8 = 1;       // Move servo to position with time
const CMD_MOVE_TIME_READ: u8 = 2;        // Read last move time and position (unused in this module)
const CMD_MOVE_START: u8 = 11;            // Execute awaited move (if using MOVE_TIME_WAIT, not used here)
const CMD_MOVE_STOP: u8 = 12;             // Stop servo movement
const CMD_ID_WRITE: u8 = 13;              // Set new servo ID
const CMD_ID_READ: u8 = 14;               // Read servo ID
const CMD_ANGLE_OFFSET_ADJUST: u8 = 17;   // Adjust angle offset (not saved to flash)
const CMD_ANGLE_OFFSET_WRITE: u8 = 18;    // Write angle offset to flash
const CMD_ANGLE_OFFSET_READ: u8 = 19;     // Read angle offset
const CMD_ANGLE_LIMIT_WRITE: u8 = 20;     // Set angle limits
const CMD_ANGLE_LIMIT_READ: u8 = 21;      // Read angle limits
const CMD_VIN_LIMIT_WRITE: u8 = 22;       // Set voltage limits
const CMD_VIN_LIMIT_READ: u8 = 23;        // Read voltage limits
const CMD_TEMP_MAX_LIMIT_WRITE: u8 = 24;  // Set max temperature limit
const CMD_TEMP_MAX_LIMIT_READ: u8 = 25;   // Read max temperature limit
const CMD_TEMP_READ: u8 = 26;             // Read current temperature
const CMD_VIN_READ: u8 = 27;              // Read current voltage (Vin)
const CMD_POS_READ: u8 = 28;              // Read current position
const CMD_OR_MOTOR_MODE_WRITE: u8 = 29;   // Switch servo (position) or motor (continuous rotation) mode
const CMD_OR_MOTOR_MODE_READ: u8 = 30;    // Read servo/motor mode status
const CMD_LOAD_OR_UNLOAD_WRITE: u8 = 31;  // Load or unload motor (enable/disable torque)
const CMD_LOAD_OR_UNLOAD_READ: u8 = 32;   // Read torque enable status

const MAX_PKT: usize = 16;          // fits every documented command   ﹡

/// Timing: 9600 baud → 1 byte ≈ 1 ms; worst-case 8-byte reply < 10 ms.
/// We give ourselves a bit of slack for relay latency in the STM32.
const WIRE_LATENCY_MS: u32 = 30;




/// A controller for LewanSoul serial bus servos (e.g. LX-16A, LX-15D) on a half-duplex UART bus.
/// 
/// This struct uses a UART interface (TX/RX) to send and receive commands to one or more serial bus servos on the same line.
/// It supports positioning servos, reading status (position, temperature, voltage), setting angle limits and torque on/off, 
/// and switching between servo (position) mode and motor (continuous rotation) mode.
/// 
/// The bus supports up to 253 servos with IDs 0-253, plus a broadcast ID 254 (0xFE) for addressing all servos&#8203;:contentReference[oaicite:3]{index=3}.
/// All communication uses 115200 baud, with a packet format of two 0x55 header bytes followed by length, command, ID, parameters, and checksum&#8203;:contentReference[oaicite:4]{index=4}.
pub struct LewanSoulBus<'a> {
    uart: UartDriver<'a>,
}

impl<'a> LewanSoulBus<'a> {
    /// Create a new LewanSoulBus controller on the given UART and pins.
    /// 
    /// # Arguments
    /// * `uart` - The UART peripheral to use (e.g. `peripherals.uart1`).
    /// * `tx_pin` - The TX pin connected to the servo bus signal line (default is GPIO32 on many ESP32 boards).
    /// * `rx_pin` - The RX pin connected to the servo bus signal line (default is GPIO33 on many ESP32 boards).
    /// * `config` - UART configuration (e.g. baud rate, should be set to 115200 baud).
    /// 
    /// # Returns
    /// A `LewanSoulBus` instance if successful, or an error if UART initialization fails.
    /// 
    /// By default the servo communication is half-duplex at 115200 bps. The TX and RX pins should be tied together to the servo signal line.
    /// This method will configure the UART without hardware flow control (CTS/RTS are not used).
    pub fn new<UART, U, TX, P1, RX, P2>(
        uart: UART,
        tx_pin: TX,
        rx_pin: RX,
        config: &Config,
    ) -> Result<Self, Error> 
    where 
        UART: Peripheral<P = U> + 'a,
        U: Uart,
        TX: Peripheral<P = P1> + 'a,
        P1: OutputPin,
        RX: Peripheral<P = P2> + 'a,
        P2: InputPin,
    {
        // Initialize the UART driver for half-duplex usage (CTS and RTS not used)
        let driver = UartDriver::new(
            uart, 
            tx_pin, 
            rx_pin, 
            Option::<AnyIOPin>::None,  // CTS pin not used
            Option::<AnyIOPin>::None,  // RTS pin not used 
            config
        )?;
        Ok(LewanSoulBus { uart: driver })
    }

    /// Move a servo to a specified angle (position) within a given time.
    /// 
    /// # Arguments
    /// * `id` - Servo ID (0-253 for specific servo, or 254 for broadcast to all servos).
    /// * `angle` - Target angle in degrees (approximately 0° to 240° range corresponds to 0-1000 position units&#8203;:contentReference[oaicite:5]{index=5}).
    /// * `time_ms` - Movement time in milliseconds. If nonzero, the servo will move to the target angle in this time (uniform speed). If 0, the servo moves as fast as possible.
    /// 
    /// # Returns
    /// `Ok(())` on success, or an error if the command failed to send.
    /// 
    /// The servo's internal position units range from 0 to 1000 for approximately 0° to 240°&#8203;:contentReference[oaicite:6]{index=6}. This function converts the given `angle` (in degrees) to the nearest position unit and sends a move command. 
    /// If broadcast ID 254 is used, all servos will move but none will return a response (to avoid bus conflict)&#8203;:contentReference[oaicite:7]{index=7}.
    pub fn move_to_angle(&mut self, id: u8, angle: f32, time_ms: u16) -> Result<(), Error> {
        // Constrain and convert angle to position units (0-1000 corresponds to 0-240 degrees approximately)
        let mut pos = (angle / 240.0 * 1000.0).round() as i16;
        if pos < 0 { pos = 0; }
        if pos > 1000 { pos = 1000; }
        self.move_to_position(id, pos as u16, time_ms)
    }

    /// Move a servo to a specified position (0-1000 units) within a given time (ms).
    /// 
    /// This is similar to [`move_to_angle`](Self::move_to_angle) but uses raw position units instead of degrees.
    pub fn move_to_position(&mut self, id: u8, position: u16, time_ms: u16) -> Result<(), Error> {
        // Prepare 4-byte parameters: position (little-endian 2 bytes) + time (little-endian 2 bytes)
        let pos_low = (position & 0x00FF) as u8;
        let pos_high = (position >> 8) as u8;
        let time_low = (time_ms & 0x00FF) as u8;
        let time_high = (time_ms >> 8) as u8;
        let params = [pos_low, pos_high, time_low, time_high];
        // Send command (no response expected for a move command)
        self.send_packet(id, CMD_MOVE_TIME_WRITE, &params, false).map(|_| ())
    }

    /// Read the current position of a servo.
    /// 
    /// # Arguments
    /// * `id` - Servo ID to read (0-253). (Broadcast ID 254 cannot be used for read commands as no response would be returned).
    /// 
    /// # Returns
    /// On success, returns the current position as a value 0-1000 (which corresponds to 0° to 240° range).
    /// Returns an error if the read fails or times out.
    /// 
    /// The position value returned can be converted to degrees if needed (approximately `position * 0.24` degrees per unit).
    pub fn read_position(&mut self, id: u8) -> Result<u16, Error> {
        // Send position read command and expect a response packet with 2-byte position
        let response = self.send_packet(id, CMD_POS_READ, &[], true)?;
        // The response packet format: [0x55, 0x55, LENGTH, CMD, ID, pos_low, pos_high, CHECKSUM]
        // We parse the position from the response.
        if response.len() >= 7 {
            let pos_low = response[5] as u16;
            let pos_high = response[6] as u16;
            let position = (pos_high << 8) | pos_low;
            Ok(position)
        } else {
            // If response is malformed or too short
            Err(anyhow::anyhow!("Malformed response from servo response.len() == {}", response.len() ))
        }
    }

    /// Enable or disable the servo motor torque (power).
    /// 
    /// # Arguments
    /// * `id` - Servo ID to control (use 254 to broadcast to all).
    /// * `enable` - true to enable torque (load the motor), false to disable torque (unload motor).
    /// 
    /// # Returns
    /// `Ok(())` on success, or an error if the command failed.
    /// 
    /// Disabling torque (unload) will stop driving the motor, letting the servo freewheel (no holding force), whereas enabling torque will allow the servo to hold position&#8203;:contentReference[oaicite:8]{index=8}.
    /// This setting does not persist after power-off.
    pub fn set_torque(&mut self, id: u8, enable: bool) -> Result<(), Error> {
        let param = if enable { 1u8 } else { 0u8 };
        self.send_packet(id, CMD_LOAD_OR_UNLOAD_WRITE, &[param], false).map(|_| ())
    }

    /// Set the minimum and maximum angle limits for a servo.
    /// 
    /// # Arguments
    /// * `id` - Servo ID to configure (0-253, 254 broadcast is not recommended for this command).
    /// * `min_angle` - Minimum allowed angle in degrees (0-240).
    /// * `max_angle` - Maximum allowed angle in degrees (0-240).
    /// 
    /// # Returns
    /// `Ok(())` on success, or an error if the command failed.
    /// 
    /// The servo will constrain its movement within the specified angle range. The provided angles will be converted to the servo's internal units (0-1000). 
    /// If either angle is out of range, it will be clamped to the valid 0-240° range.
    pub fn set_angle_limits(&mut self, id: u8, min_angle: f32, max_angle: f32) -> Result<(), Error> {
        // Convert degrees to 0-1000 units and clamp
        let mut min_pos = (min_angle / 240.0 * 1000.0).round() as i32;
        let mut max_pos = (max_angle / 240.0 * 1000.0).round() as i32;
        if min_pos < 0 { min_pos = 0; }
        if max_pos < 0 { max_pos = 0; }
        if min_pos > 1000 { min_pos = 1000; }
        if max_pos > 1000 { max_pos = 1000; }
        if min_pos > max_pos {
            core::mem::swap(&mut min_pos, &mut max_pos);
        }
        let min = min_pos as u16;
        let max = max_pos as u16;
        let params = [
            (min & 0xFF) as u8,
            (min >> 8) as u8,
            (max & 0xFF) as u8,
            (max >> 8) as u8,
        ];
        self.send_packet(id, CMD_ANGLE_LIMIT_WRITE, &params, false).map(|_| ())
    }

    /// Set the operating mode of the servo: positional (servo) mode or continuous rotation (motor) mode.
    /// 
    /// # Arguments
    /// * `id` - Servo ID to configure.
    /// * `motor_mode` - If true, enable continuous rotation mode (motor mode). If false, enable standard servo position mode.
    /// * `speed` - In motor mode, the speed value (-1000 to 1000) for rotation. Positive values for one direction, negative for the opposite. Ignored in servo mode.
    /// 
    /// # Returns
    /// `Ok(())` on success, or an error if the command failed.
    /// 
    /// In motor mode, the servo will not hold position but rotate continuously at the given speed. In servo mode, the servo holds its target position and the speed parameter is ignored.
    /// The speed is specified as a signed value; it will be converted to the protocol format (two's complement) for transmission.    
    pub fn set_mode(&mut self, id: u8, motor_mode: bool, speed: i16) -> Result<(), Error> {
        let mode_byte = if motor_mode { 1u8 } else { 0u8 };
        // The protocol expects a 4-byte parameter sequence: mode (0 or 1), a "zero" byte (unused), speed low, speed high.
        let speed_value = speed as u16; // interpret the i16 as unsigned 16-bit (two's complement representation for negative values)
        let params = [
            mode_byte,
            0u8, 
            (speed_value & 0xFF) as u8,
            (speed_value >> 8) as u8
        ];
        self.send_packet(id, CMD_OR_MOTOR_MODE_WRITE, &params, false).map(|_| ())
    }

    pub fn send_packet(
        &mut self,
        id: u8,
        command: u8,
        params: &[u8],
        want_reply: bool,
    ) -> anyhow::Result<Vec<u8>> {
        /* ---------- 1 · Format packet ---------- */
        let mut tx: [u8; MAX_PKT] = [0; MAX_PKT];
        let len: u8 = 3 + params.len() as u8;   // CMD+ID+PARAMS+CHK
        
        // Build the packet: header + ID + length + command + params + checksum
        let mut idx = 0;
        tx[idx..idx + 2].copy_from_slice(&[0x55, 0x55]); idx += 2; // Header bytes
        tx[idx] = id;                  idx += 1;
        tx[idx] = len;                 idx += 1;
        tx[idx] = command;             idx += 1;
        tx[idx..idx + params.len()].copy_from_slice(params); idx += params.len();
    
        // Calculate checksum: bitwise-NOT of sum(ID+LEN+CMD+PARAMS)
        let chk = !(tx[2..idx].iter().fold(0u8, |s, b| s.wrapping_add(*b)));
        tx[idx] = chk; idx += 1;
        let tx_len = idx;
    
        println!("Sending packet: {:?}", &tx[..tx_len]);
        
        /* ---------- 2 · Send packet with flush ---------- */
        // Clear RX buffer to remove any stale data
        self.uart.clear_rx()?;
        
        // Write data and ensure it's sent completely
        self.uart.write(&tx[..tx_len])?;
        
        // For daisy-chained servos with a single TX/RX cable, we need to handle echo
        // Since we're using the same wire for TX and RX, we'll see an echo of what we send
        
        // If no reply expected, we're done after sending
        if !want_reply {
            return Ok(Vec::new());
        }
        
        // For a half-duplex single-wire connection, we need to read and discard our own echo
        // Read exactly the number of bytes we sent to clear them from the buffer
        let mut echo_buf = [0u8; MAX_PKT];
        let mut echo_bytes_read = 0;
        
        // Read until we've consumed our echo or timed out
        while echo_bytes_read < tx_len {
            match self.uart.read(&mut echo_buf[echo_bytes_read..tx_len], 5) {
                Ok(n) if n > 0 => {
                    echo_bytes_read += n;
                },
                _ => break, // If we can't read more, just continue to actual response
            }
        }
        
        println!("Read and discarded {} echo bytes", echo_bytes_read);
        
        // We've already checked if reply is expected above

        /* ---------- 3 · Read response with timeout ---------- */
        let mut rx: [u8; MAX_PKT] = [0; MAX_PKT];
        let mut bytes_read = 0;
        let timeout_ms = 100; // 100ms timeout for complete response - increased for daisy-chain
        
        // Read header first (at least 4 bytes: 2x55 + ID + LEN)
        let start_time = unsafe { esp_timer_get_time() } / 1000;
        while bytes_read < 4 {
            if (unsafe { esp_timer_get_time() } / 1000) - start_time > timeout_ms {
                return Err(anyhow::anyhow!("Timeout waiting for response header"));
            }
            
            match self.uart.read(&mut rx[bytes_read..], 10) {
                Ok(n) if n > 0 => {
                    bytes_read += n;
                    println!("Read {} bytes, current buffer: {:?}", n, &rx[..bytes_read]);
                    // Look for header pattern once we have enough bytes
                    if bytes_read >= 2 && !(rx[0] == 0x55 && rx[1] == 0x55) {
                        // If we don't see the header pattern, shift and try again
                        rx.copy_within(1..bytes_read, 0);
                        bytes_read -= 1;
                    }
                },
                Ok(_) => println!("No bytes read, trying again..."), // No bytes read, try again
                Err(e) => println!("UART read error: {} but continue", e),
            }
        }
        
        // Now process the response (already in the rx buffer)
        // Calculate full packet length based on the header we've read
        let len_field = rx[3] as usize;  // LEN field (includes CMD+params+CHK)
        let frame_len = 3 + len_field;   // Full packet length
        
        if frame_len > MAX_PKT {
            anyhow::bail!("Response size {} exceeds buffer size {}", frame_len, MAX_PKT);
        }
        
        // Read the rest of the packet (command + params + checksum)
        // For daisy-chained servos, we need a more generous timeout
        let packet_timeout_ms = 200; // Longer timeout for reading the complete packet
        let packet_start_time = unsafe { esp_timer_get_time() } / 1000;
        
        while bytes_read < frame_len {
            if (unsafe { esp_timer_get_time() } / 1000) - packet_start_time > packet_timeout_ms {
                // If we've got a partial packet but timed out, log what we have and return error
                println!("Partial packet received: {:?}", &rx[..bytes_read]);
                anyhow::bail!("Timeout reading complete packet: got {} of {} bytes", 
                             bytes_read, frame_len);
            }
            
            // Use a very short timeout to poll efficiently but frequently
            match self.uart.read(&mut rx[bytes_read..frame_len], 5) {
                Ok(n) if n > 0 => bytes_read += n,
                Ok(_) => continue, // No bytes read, try again
                Err(e) => return Err(anyhow::anyhow!("UART read error: {}", e))
            }
        }
        
        println!("Received response: {:?}", &rx[..frame_len]);
        
        /* ---------- 4 · Validate checksum ---------- */
        let chk_rx = rx[frame_len - 1];
        let chk_calc = !(rx[2..frame_len - 1].iter().fold(0u8, |s, b| s.wrapping_add(*b)));
        if chk_rx != chk_calc {
            anyhow::bail!("Checksum error: received 0x{:02X}, calculated 0x{:02X}", chk_rx, chk_calc);
        }
    
        Ok(rx[..frame_len].to_vec())
    }
}