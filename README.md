# sinodude

A programming tool for SinoWealth 8051 microcontrollers.

## Warning

**This tool is experimental. Use at your own risk.**

SinoWealth microcontrollers have limited public documentation. This tool has been developed through reverse engineering and may not work correctly with all chip variants and all configurations.

## Supported Programmers

| Programmer | Description | Notes |
|------------|-------------|-------|
| sinodude-serial | Open-source Arduino Nano (ATmega328P or ATmega328PB) based programmer. See [firmware/README.md](firmware/README.md) for details. | Recommended |

## Recovery Workflow (Windows, SH68F90A/SH68F90)

This section documents a practical recovery flow used on BYK901/SH68F90A/SH68F90 boards.
It is based on a real blank-IC replacement case on an EWEADN V20 board (Redragon K618 family), where the original controller was marked `BYK901` (remarked `SH68F90A`).
In that case, the original IC had ESD damage on one key matrix/grid input, which required replacing the MCU.
In this specific repair, the replacement chip was another `BYK901`-marked part but identified as non-`A` (`SH68F90`), so flashing with `sh68f90a` initially failed with part-mismatch errors.

### 0) Read original dump first ([sinowealth-kb-tool](https://github.com/carlossless/sinowealth-kb-tool))

Before any erase/replacement/write, make ISP backups with `sinowealth-kb-tool`.

Use the platform-based flow first (recommended for remarked/unknown boards):

```powershell
# 1) identify VID:PID of the connected keyboard
sinowealth-kb-tool list

# 2) backup firmware region
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x<VID> --product_id 0x<PID> -s firmware .\dumps\dump.hex

# 3) backup bootloader region
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x<VID> --product_id 0x<PID> -s bootloader .\dumps\bootloader.hex

# 4) mandatory golden backup (firmware + bootloader)
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x<VID> --product_id 0x<PID> -s full .\dumps\full.hex
```

Notes:
- valid platform values are `sh68f90` and `sh68f881`.
- if your board is in this SH68F90 family, start with `--platform sh68f90`.

Verified example on this board:

```text
sinowealth-kb-tool list -> ID 05ac:024f manufacturer="SINO WEALTH" product="Gaming KB "
```

```powershell
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x05ac --product_id 0x024f -r 1 -s firmware .\dumps\dump.hex
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x05ac --product_id 0x024f -r 1 -s bootloader .\dumps\bootloader.hex
sinowealth-kb-tool read --platform sh68f90 --vendor_id 0x05ac --product_id 0x024f -r 1 -s full .\dumps\full.hex
```

For predefined `device_name` profiles and advanced options, see the `sinowealth-kb-tool` docs: https://github.com/carlossless/sinowealth-kb-tool#usage

### 1) Required software

- Rust toolchain:
  - stable (host `sinodude`)
  - nightly + `rust-src` (firmware)
- Visual Studio 2026 Build Tools (Windows host build):
  - `Desktop development with C++`
  - `MSVC toolset` (for your VS installation)
  - `Windows 11 SDK`
- MSYS2 AVR toolchain:
  - `avr-gcc`, `avr-binutils`, `avr-libc`, `avrdude`
- `ravedude` (used by the firmware runner setup)
- `sinowealth-kb-tool` (for ISP backup/restore workflow)

### 2) Build and flash Arduino programmer (sinodude-serial)

Build host tool:

```powershell
cd c:\path\to\sinodude
cargo build --release
```

Build firmware:

```powershell
cd c:\path\to\sinodude\firmware
cargo build --release
```

Flash Uno/Nano (ATmega328P bootloader example):

```powershell
C:\msys64\ucrt64\bin\avrdude.exe -p m328p -c arduino -P COM5 -b 115200 -D -U flash:w:c:\path\to\sinodude\firmware\target\avr-none\release\sinodude-serial.elf:e
```

`COM5` is only an example. Your board may enumerate as a different port (`COM3`, `COM7`, etc.).  
Use the COM port associated with your Arduino board.

### 3) Capture part number / operation number / code options from console

Run one `read` and keep the console output:

```powershell
cd c:\path\to\sinodude
.\target\release\sinodude.exe read -c sinodude-serial -p sh68f90a --port COM5 .\dumps\probe.hex
```

Capture these lines from output:

```text
Target Part Number: 68f90a0000
Operation Number: 56c9
Security Bits: 3000300030003000000000000000000000
Code Options: a4e063c00f000088
```

**IMPORTANT:** Do not skip this step. Save `Target Part Number`, `Operation Number`, and `Code Options` exactly as printed.  
You will need them in [Step 4](#4-write-full-dump-with-proper-args) and in [Troubleshooting: Keyboard boots but Windows shows "USB device not recognized"](#keyboard-boots-but-windows-shows-usb-device-not-recognized).

### 4) Write full dump with proper args

Use the full dump (`full.hex`) and set explicit security/custom profile.
Use the actual part detected on your target (`sh68f90a` or `sh68f90`).

```powershell
.\target\release\sinodude.exe write -c sinodude-serial -p sh68f90 --port COM5 .\dumps\full.hex --security 0000000000000000000000000000000000 --customer_option a4e063c00f000088 --operation_number 56c9
```

Read back and verify:

```powershell
.\target\release\sinodude.exe read -c sinodude-serial -p sh68f90 --port COM5 .\dumps\verify.hex
Get-FileHash .\dumps\verify.hex -Algorithm SHA256
Get-FileHash .\dumps\full.hex -Algorithm SHA256
```

## Troubleshooting

### `Pinging programmer...` timeout

Typical cause is bad/empty Arduino firmware image or COM port contention.

1. Check firmware size:
```powershell
C:\msys64\ucrt64\bin\avr-size.exe c:\path\to\sinodude\firmware\target\avr-none\release\sinodude-serial.elf
```
2. If `.text` is suspiciously tiny (for example around `150` bytes), rebuild firmware.
3. Reflash Arduino firmware:
```powershell
C:\msys64\ucrt64\bin\avrdude.exe -p m328p -c arduino -P COM5 -b 115200 -D -U flash:w:c:\path\to\sinodude\firmware\target\avr-none\release\sinodude-serial.elf:e
```

Reference from this recovery:
- bad image symptom: tiny firmware (`.text ~150` bytes)
- working image: normal firmware size (`.text ~13960` bytes)

### Part mismatch errors (`sh68f90a` vs `sh68f90`)

Always trust the part reported by read output and match `-p` to that value.

Example mapping:
- `Target Part Number: 68f90a0000` -> use `-p sh68f90a`
- `Target Part Number: 68f9000000` -> use `-p sh68f90`


### Verify failures near low addresses (`0x0000` range)

If write/verify fails near low addresses, write again with explicit zero security:

```powershell
.\target\release\sinodude.exe write -c sinodude-serial -p sh68f90 --port COM5 .\dumps\full.hex --security 0000000000000000000000000000000000
```

Then read back and hash-check:

```powershell
.\target\release\sinodude.exe read -c sinodude-serial -p sh68f90 --port COM5 .\dumps\verify.hex
Get-FileHash .\dumps\verify.hex -Algorithm SHA256
Get-FileHash .\dumps\full.hex -Algorithm SHA256
```

Important detail:
- `Security Bits` shown during read (for example `3000300030003000000000000000000000`) are the current chip state.
- In this recovery path, reliable write used:
  - `--security 0000000000000000000000000000000000`

### Keyboard boots but Windows shows "USB device not recognized"

This usually means firmware runs but option/profile fields are wrong for USB behavior.

**IMPORTANT:** Use the values captured in [Step 3: Capture part number / operation number / code options from console](#3-capture-part-number--operation-number--code-options-from-console).  
Do not guess `--customer_option` or `--operation_number`.

Re-write with captured original-chip values:

```powershell
.\target\release\sinodude.exe write -c sinodude-serial -p sh68f90 --port COM5 .\dumps\full.hex --security 0000000000000000000000000000000000 --customer_option a4e063c00f000088 --operation_number 56c9
```

After write, confirm the values by reading once and checking console output:
- `Operation Number: 56c9`
- `Code Options: a4e063c00f000088`

### `sinodude` dump differs from ISP full dump

If a `sinodude read` dump does not restore correctly, use `sinowealth-kb-tool -s full` as golden backup.

Recommended backup policy:
1. Keep ISP `full.hex` as primary recovery image.
2. Keep `sinodude` read(s) as secondary diagnostics.
3. Keep at least one captured read console log with:
   - `Target Part Number`
   - `Operation Number`
   - `Code Options`
   - `Security Bits`

### Intermittent detect/read failures

Most common causes are physical connection issues.

Check:
- continuity and solder quality on `TDO`, `TMS`, `TDI`, `TCK`, `GND`, `VDD`
- stable common ground between programmer and target
- stable target power switching path (PMOS high-side, gate pull-up, D6 control)
- connector/contact pressure after chip rework
