# SinoDude AVR Programmer Firmware

AVR-based programmer firmware for SinoWealth 8051 microcontrollers using the ICP (In-Circuit Programming) protocol.

## Supported Boards

- Arduino Uno (ATmega328P)
- Other ATmega328P-based boards

## Pin Connections

| AVR Pin | Function | SinoWealth Target |
|---------|----------|-------------------|
| D2      | TDO      | Test Data Out     |
| D3      | TMS      | Test Mode Select  |
| D4      | TDI      | Test Data In      |
| D5      | TCK      | Test Clock        |
| D6      | POWER    | VCC (via switch)  |
| GND     | GND      | Ground            |

### Wiring Diagram

```
Arduino Uno          SinoWealth MCU
-----------          --------------
   D2  <------------>  TDO
   D3  ------------->  TMS
   D4  ------------->  TDI
   D5  ------------->  TCK
   D6  ----[MOSFET]-->  VCC (power control)
   GND <------------>  GND
```

For power control, you can use a P-channel MOSFET or a logic-level relay connected to D6.

## Serial Protocol

The firmware communicates with the host at 115200 baud (8N1).

### Commands

| Command | Byte | Description |
|---------|------|-------------|
| PING    | 0x01 | Check programmer is alive, returns 0x00 + "SW" |
| CONNECT | 0x02 | Connect to target MCU via ICP |
| DISCONNECT | 0x03 | Disconnect from target MCU |
| READ_FLASH | 0x04 | Read flash memory |
| WRITE_FLASH | 0x05 | Write flash memory |
| ERASE_FLASH | 0x06 | Erase flash sector |
| POWER_ON | 0x07 | Power on target |
| POWER_OFF | 0x08 | Power off target |

### Response Codes

| Response | Byte | Description |
|----------|------|-------------|
| OK       | 0x00 | Operation successful |
| DATA     | 0x01 | Data response follows |
| ERROR    | 0xFF | Operation failed |

### Packet Format

**READ_FLASH command:**
```
[CMD:1][ADDR:4 LE][LEN:2 LE]
```

**READ_FLASH response:**
```
[RSP_DATA:1][LEN:2 LE][DATA:LEN]
```

**WRITE_FLASH command:**
```
[CMD:1][ADDR:4 LE][LEN:2 LE][DATA:LEN]
```

**ERASE_FLASH command:**
```
[CMD:1][ADDR:4 LE]
```

## Building

Requires Rust nightly with AVR support:

```bash
cd firmware
rustup override set nightly
cargo build --release
```

The output ELF file will be in `target/avr-atmega328p/release/sinodude-firmware.elf`.

## Flashing

Using avrdude:

```bash
avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:target/avr-atmega328p/release/sinodude-firmware.elf
```

Or with ravedude (if installed):

```bash
cargo run --release
```

## Usage with sinodude

```bash
# Read flash
sinodude read output.hex -c serial --port /dev/ttyUSB0 -p 68f90a -t external

# Write flash
sinodude write firmware.hex -c serial --port /dev/ttyUSB0 -p 68f90a -t external
```

## References

- [sinowealth-8051-bl-updater](https://github.com/gashtaan/sinowealth-8051-bl-updater) - Reference implementation
