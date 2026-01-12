# SinoDude Serial Programmer Firmware

AVR-based programmer firmware for SinoWealth 8051 microcontrollers using the ICP protocol.

## Supported Boards

- Arduino Nano (ATmega328P)
- Arduino Nano Every compatible boards (ATmega328PB)

## Pin Connections

| Arduino Nano Pin | Function | SinoWealth Target |
|------------------|----------|-------------------|
| D2               | TDO      | Test Data Out     |
| D3               | TMS      | Test Mode Select  |
| D4               | TDI      | Test Data In      |
| D5               | TCK      | Test Clock        |
| D6               | POWER    | VCC (via MOSFET)  |
| GND              | GND      | Ground            |

## Power Control Circuit

The programmer controls power to the target using a P-channel MOSFET. This allows proper power cycling which is required for entering ICP mode.

```
                VCC
                 |
                [R] 10K
                 |
    D6 ----+----'
           |
           G
        |--+--|
  VCC --|S   D|----> Target VCC
        |-----|
         PMOS
```

**Components:**
- P-channel MOSFET (e.g., IRFU9024, or similar logic-level MOSFET)
- 10K resistor (pull-up from gate to VCC)

**How it works:**
- When D6 is LOW, gate is pulled low, MOSFET turns ON, power flows to target
- When D6 is HIGH (or floating), gate is pulled high via 10K resistor, MOSFET turns OFF
- The 10K pull-up ensures the MOSFET stays OFF during Arduino reset

## Serial Protocol

The firmware communicates with the host at 115200 baud (8N1).

### Commands

| Command      | Byte | Description                              |
|--------------|------|------------------------------------------|
| PING         | 0x01 | Check programmer is alive, returns "SW"  |
| GET_VERSION  | 0x02 | Get firmware version (major, minor)      |
| POWER_ON     | 0x03 | Power on target                          |
| POWER_OFF    | 0x04 | Power off target                         |
| CONNECT      | 0x05 | Connect to target MCU via ICP            |
| DISCONNECT   | 0x06 | Disconnect from target MCU               |
| GET_ID       | 0x07 | Get target JTAG ID                       |
| SET_CONFIG   | 0x08 | Set chip type configuration              |
| GET_CONFIG   | 0x09 | Get current chip type configuration      |
| READ_FLASH   | 0x0A | Read flash memory                        |
| WRITE_FLASH  | 0x0B | Write flash memory                       |
| ERASE_FLASH  | 0x0C | Erase flash sector                       |

### Response Codes

| Response | Byte | Description              |
|----------|------|--------------------------|
| OK       | 0x00 | Operation successful     |
| DATA     | 0x01 | Data response follows    |
| ERROR    | 0xFF | Operation failed         |

## Building

Requires Rust nightly with AVR support:

```bash
cd firmware
rustup override set nightly
```

### ATmega328P (default)

```bash
RUSTFLAGS="-C target-cpu=atmega328p" cargo build --release
```

The output ELF file will be in `target/avr-none/release/sinodude-serial.elf`.

### ATmega328PB

```bash
RUSTFLAGS="-C target-cpu=atmega328pb" cargo build --release --no-default-features --features atmega328pb
```

## Flashing

Using avrdude for ATmega328P:

```bash
avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:target/avr-none/release/sinodude-serial.elf
```

Using avrdude for ATmega328PB:

```bash
avrdude -p atmega328pb -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:target/avr-none/release/sinodude-serial.elf
```

## Acknowledgments

The sinodude-serial programmer was made possible thanks to the reverse engineering work by [gashtaan](https://github.com/gashtaan) and his projects:

- [sinowealth-8051-dumper](https://github.com/gashtaan/sinowealth-8051-dumper) - Flash dumper for SinoWealth 8051 MCUs
- [sinowealth-8051-bl-updater](https://github.com/gashtaan/sinowealth-8051-bl-updater) - Bootloader updater and ICP protocol implementation
