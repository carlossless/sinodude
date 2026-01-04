# SinoDude

A programming tool for SinoWealth 8051 microcontrollers.

## Warning

**This tool is experimental. Use at your own risk.**

SinoWealth microcontrollers have limited public documentation. This tool has been developed through reverse engineering and may not work correctly with all chip variants. Incorrect usage could potentially damage your hardware or brick your device.

## Supported Parts

| Part | Flash Size | Notes |
|------|------------|-------|
| SH68F90A | 64KB | Tested |

## Programmers

SinoDude supports two programmer backends:

### sinolink

The official SinoLink USB programmer. Requires the `--power` option to set voltage (3v3, 5v, or external).

### sinodude-serial

An open-source Arduino-based programmer. See the [firmware](firmware/) directory for the AVR firmware.

## sinodude-serial Wiring

The sinodude-serial programmer uses an Arduino Nano (ATmega328P) to communicate with the target SinoWealth MCU via the ICP (In-Circuit Programming) protocol.

### Pin Connections

| Arduino Nano Pin | Function | SinoWealth Target |
|-------------|----------|-------------------|
| D2          | TDO      | Test Data Out     |
| D3          | TMS      | Test Mode Select  |
| D4          | TDI      | Test Data In      |
| D5          | TCK      | Test Clock        |
| D6          | POWER    | VCC (via MOSFET)  |
| GND         | GND      | Ground            |

### Power Control Circuit

The programmer controls power to the target using a P-channel MOSFET. This allows proper power cycling which is required for entering ICP mode.

```
        VCC (5V or 3.3V)
            |
            |
           _|_
          |   |  P-Channel MOSFET
    D6 ---| G |  (e.g., SI2301, AO3401)
          |___|
            |
            S
            |
            +-------> To Target VCC
            |
           [ ] 10K Pull-up Resistor
            |
           VCC
```

**Components:**
- P-channel MOSFET (e.g., SI2301, AO3401, or similar logic-level MOSFET)
- 10K resistor (pull-up from gate to VCC)

**How it works:**
- When D6 is LOW, the MOSFET turns ON, supplying power to the target
- When D6 is HIGH, the MOSFET turns OFF, cutting power to the target
- The 10K pull-up resistor ensures the MOSFET stays OFF when the Arduino is resetting or not actively driving the pin

### Complete Wiring Diagram

```
Arduino Nano                             SinoWealth MCU
------------                             --------------

                    VCC
                     |
                    [ ] 10K
                     |
    D6 -------------+----G
                         |
                    [PMOS]
                         |
                         S
                         +---------------> VCC

    D2 <-------------------------> TDO
    D3 --------------------------> TMS
    D4 --------------------------> TDI
    D5 --------------------------> TCK
    GND <------------------------> GND
```

## Installation

Download pre-built binaries from the [Releases](https://github.com/carlossless/sinodude/releases) page, or build from source:

```bash
cargo build --release
```

## Usage

### Reading Flash

```bash
# Using sinolink
sinodude read output.hex -c sinolink -p 68f90a --power 3v3

# Using sinodude-serial
sinodude read output.hex -c sinodude-serial -p 68f90a --port /dev/ttyUSB0
```

### Writing Flash

```bash
# Using sinolink
sinodude write firmware.hex -c sinolink -p 68f90a --power 3v3

# Using sinodude-serial
sinodude write firmware.hex -c sinodude-serial -p 68f90a --port /dev/ttyUSB0
```

### Decrypting GPT Files

SinoWealth's official programming tool uses encrypted `.gpt` firmware files:

```bash
sinodude --decrypt firmware.gpt
```

## Building the sinodude-serial Firmware

See [firmware/README.md](firmware/README.md) for instructions on building and flashing the Arduino firmware.

## Acknowledgments

The sinodude-serial programmer was made possible thanks to the reverse engineering work by [gashtaan](https://github.com/gashtaan):

- [sinowealth-8051-dumper](https://github.com/gashtaan/sinowealth-8051-dumper) - Flash dumper for SinoWealth 8051 MCUs
- [sinowealth-8051-bl-updater](https://github.com/gashtaan/sinowealth-8051-bl-updater) - Bootloader updater and ICP protocol implementation
