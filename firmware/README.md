# SinoDude Serial Programmer Firmware

AVR-based programmer firmware for SinoWealth 8051 microcontrollers using the ICP protocol.

## Supported Controllers

- ATmega328P ([Arduino Nano](https://docs.arduino.cc/hardware/nano/))
- ATmega328PB (Arduino Nano derivatives)

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

The programmer controls power to the target using a P-channel MOSFET. This allows power cycling on command which is required for entering ICP mode.

```
     VCC -------+--------+
(Arduino 5V)    |        |
             [ 10k ] +-- S --+
                |    |       |
     D6 --------+--- G       D --------> Target VCC
                     | P-MOS |
                     +-------+
```

**Components:**
- P-channel MOSFET (e.g., IRF9540, IRFU9024, or similar logic-level MOSFET)
- 10K resistor (pull-up from gate to source)

**How it works:**
- When D6 is LOW, Vgs becomes negative, MOSFET turns ON, power flows to target
- When D6 is HIGH (or floating), gate is pulled to source via 10K resistor (Vgs=0), MOSFET turns OFF
- The 10K resistor ensures the MOSFET stays OFF during Arduino reset

## Acknowledgments

The sinodude-serial programmer wouldn't have been possible if not for the reverse engineering work by [gashtaan](https://github.com/gashtaan) and his open-source projects:

- [sinowealth-8051-dumper](https://github.com/gashtaan/sinowealth-8051-dumper) - Flash dumper for SinoWealth 8051 MCUs
- [sinowealth-8051-bl-updater](https://github.com/gashtaan/sinowealth-8051-bl-updater) - Bootloader updater and ICP protocol implementation
