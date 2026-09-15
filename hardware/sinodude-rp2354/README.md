# sinodude-rp2354

A USB ICP programmer for SinoWealth 8051 microcontrollers, built on an RP2354A.

The target-facing side is switchable between 3.3 V and 5 V, level-translated, and can be
actively discharged, so the host can power-cycle a target cleanly. That matters: SinoWealth
parts only enter ICP mode on a power-on sequence, and a target rail that decays slowly
through its own decoupling will not re-trigger it.

The probe header is pin-compatible with the SinoWealth SinoLink V2.2, so existing cables
and adapters work unchanged.

## Status

| | |
|---|---|
| Schematic | complete — 93 components, ERC 0 errors / 8 warnings (see below) |
| Sourcing | 88 of 93 parts carry an `LCSC` field; the 5 without are mounting holes and Tag-Connect pads |
| PCB | not started — no outline, no placement, no routing |

## Sourcing

Every component has a footprint. LCSC part numbers are in an `LCSC` field on each symbol,
picked from the JLCPCB catalogue, preferring Basic/Preferred parts where one exists.

### Value fields carry the binding spec

Parts whose datasheet imposes a real constraint spell it out in the Value field, Olimex
style, so the requirement travels with the symbol instead of living only in this README:

```
L1   3.3uH/20%/Isat2.4A/DCR140m/POL      C22,C23  15pF/50V/5%/C0G/C0603
C19  10uF/6.3V/20%/X5R/C0402             R15      750R/1%/ILIM333mA
C20,C21,C37  4.7uF/10V/20%/X5R/C0402     R21      150R/1%/250mW/R1206
R5   33R/1%/R0402                        F1       500mA/Rmax300mR/F1206
Y1   12MHz/CL10pF/ESR50R/ABM8-272-T3      D4-D8    H5VSD3B/VRWM5V/SOD-323
```

Generic 10 k / 100 nF parts keep plain values — stamping specs on those only fragments the
BOM without protecting anything.

### Core regulator parts, against RP2350 datasheet §6.3.8

The datasheet opens that section with *"this should be placed first on any board design and
these guidelines must be strictly followed"*, and gives hard numbers. What is fitted:

| Requirement | Part | Margin |
|---|---|---|
| L: shielded, 3.3 µH ±20%, DCR ≤ 250 mΩ, I_sat ≥ 1.5 A, polarity-marked | AOTA-B201610S3R3-101-T (`C42411119`) | DCR 140 mΩ max, I_sat 2.4 A min ✓ |
| C_OUT: 4.7 µF ±20%, ESR ≤ 250 mΩ, ESL ≤ 6 nH | C21, 0402 4.7 µF 10 V X5R ±20% (`C23733`) | on 1.1 V, so negligible DC-bias loss |
| C_IN: ≥ 4.7 µF, ESR ≤ 50 mΩ | C19, 0402 **10 µF** 6.3 V X5R (`C15525`) | 10 µF chosen because a 4.7 µF part derates below the floor at 3.3 V bias |
| Second 4.7 µF on V_OUT near DVDD pin 23 (QFN-60) | C37 (`C23733`) | recommended, not mandatory |

**C37 must be placed at the bottom edge of U1 near DVDD pin 23 and away from L1/C21** — the
datasheet is explicit that it should not sit near the LX/C_OUT loop. There is a note on the
sheet to that effect.

The inductor's polarity marking is functional, not cosmetic: the datasheet says leakage field
couples into the LX→L→C_OUT loop and affects the control loop and output voltage, which is
why Raspberry Pi had Abracon make a marked part. L1's footprint puts the dot on pad 1, which
must face VREG_LX.

L1 and C21 can only be deleted by powering DVDD from an external 1.1 V supply, disabling the
on-chip regulator and tying VREG_FB to ground — which costs a whole extra regulator, so it is
not worth it here. VREG_AVDD and VREG_VIN still need power in that case regardless, as they
run the power-on reset and brown-out detector.

**U3 (TPS2114A) is `C118265`, listed but currently at zero stock.** It is orderable rather
than absent, and at $1.92 it is the cheapest of its family. TI lists the part ACTIVE. The
only consequence is scheduling: a zero-stock part cannot come off JLCPCB's shelf for
assembly, so it has to be ordered against LCSC lead time or consigned.

**Do not substitute the TPS2115A for it.** The two share a datasheet and a pinout, and
differ in exactly three ways: current-limit range (0.31–0.75 A vs 0.63–2 A), the ILIM
constant (250 vs 500), and r_DS(on) (120 mΩ vs 84 mΩ). Our 333 mA sits mid-range for the
'2114A and *below the bottom* of the '2115A's, where the datasheet notes the minimum is set
"based on accuracy considerations" — i.e. the limit is not characterised there. Getting
333 mA from a '2115A needs R15 = 1.5 kΩ, outside its range; staying inside its range means
≥630 mA to the target, which breaks the 500 mA USB budget. R15's value is bound to the part
choice, which is why its Value field reads `750R/1%/ILIM333mA`.

Left without an LCSC — neither is a part:

| Ref | Why |
|---|---|
| H1–H4 | mounting holes, drilled features rather than components |
| J2 | Tag-Connect TC2030 is bare pads — the pogo cable touches copper |

Every part that is physically placed on the board has an LCSC number. J1, J4, SW1 and SW2
all resolved to exact matches for the footprints already drawn:

| Ref | Part | LCSC | Note |
|---|---|---|---|
| J1 | HCTL HC-TYPE-C-16P-01A | `C2894897` | the part the KiCad footprint is named after |
| J4 | ZHOURI DC3-2.54-10PAL | `C5156674` | right-angle shrouded box header |
| SW1, SW2 | C&K KMR221GLFS | `C72443` | the part the KiCad footprint is named after |

**Watch the DC3 suffix when reordering J4.** The family splits on one letter: `-10PAS` is
`Plugin`, i.e. vertical, and `-10PAL` is `Push-Pull`, i.e. **Right Angle**. Only the PAL
fits `IDC-Header_2x05_P2.54mm_Horizontal`. Ordering the PAS gives you a connector that
physically fits the same holes but points the cable straight up off the board.

**U2 is now an AMS1117-3.3** (`C6186`, Basic, ~1.4 M in stock), not the NCP1117 the RP2350
guide uses — NCP1117 is not stocked at JLCPCB. Same SOT-223 land pattern, same pinout
(1 GND, 2 VO, 3 VI).

Two part-choice consequences worth knowing:

- **R9 is too large for a green LED.** A green 0603 has Vf ≈ 2.6–2.9 V, so 470 Ω from 3.3 V
  gives under 1 mA and a barely visible LED. Drop R9 to ~150 Ω. R10 is fine — the red LED's
  Vf ≈ 2.0 V gives ~2.8 mA.
- **C27 (10 µF on VTGT) slows supply switching.** The TPS211xA will not connect a supply
  until OUT has fallen within 100 mV of it, so on a 5 V → 3.3 V change VTGT must decay to
  ~3.4 V first. TI notes the fast input-to-input slew "reduces the output voltage hold-up
  capacitance requirement". Q1 is therefore not optional for voltage changes, not just for
  ICP power cycling — or drop C27 to 1–4.7 µF.

### Known items

- **8 ERC warnings**, all the same: U4's unused pins (7–10 = A5–A8, 14–17 = B5–B8) are tied
  to GND, and GND carries a PWR_FLAG, so KiCad flags "bidirectional pin connected to power
  output". This is correct — exclude the warnings, don't rewire. KiCad only sees the pin
  *type*; it cannot know DIR is strapped.

  **Why both ports are grounded, not just the A side.** The SN74LVC8T245 datasheet §8.1 says
  *"It is recommended to tie all unused I/Os to GND. The device should not have any floating
  I/Os when changing translation direction"*, and the overview is blunter: *"The input
  circuitry on both A and B ports is always active and must have a logic HIGH or LOW level
  applied to prevent excess ICC and ICCZ."* The input receiver stays powered on a pin even
  while that pin is driven as an output, so a floating B pin still presents a floating CMOS
  input internally and still burns ICC. Every A and B pin is typed `I/O` in the pin table,
  not "output".

  Grounding an output is only safe because **DIR is hardwired to VCCA**, so the A→B direction
  never changes and B5–B8 sit driving low into GND — no contention, no current. If DIR ever
  became firmware-driven, both sides would need series resistors instead of hard ties. There
  is a note on the sheet saying so.
- **Schematic style follows Olimex.** Ten titled blocks, each boxed with dashed graphic lines
  (`polyline` notes lines, the same construct Olimex uses). Related parts sit next to each
  other and are joined with real wires; KiCad power symbols carry every rail; global labels
  are used only for signals that cross from one block to another.
- **`VTGT`, `ADC_AVDD` and `VREG_AVDD` are real power symbols**, not labels. KiCad takes a
  power symbol's net name from its Value field, so these are stock symbols (`power:VCC`,
  `power:+3.3VA`) with the Value overridden. Keep the Value if you copy them.
- **D1 is wired as a pass-through**, not as a shunt with its two I/O1 pins tied together.
  D+/D- enter one side of the USBLC6 and leave the other, so the signal runs through the
  diode pads rather than past them on a stub. That is ST's application circuit and gives the
  clamp a proper current path — but it does mean **D1 cannot be left unpopulated**.
- **`KEY` (header pin 7) function is unknown.** On SinoLink it is an RC-filtered sense line
  that was never traced to the MCU. It gets its own translator with a live DIR plus the same
  RC (R32/C36) so firmware can treat it as input or output later.
- **U1 uses the ThermalVias QFN footprint** (vias under the exposed pad), so board setup min
  hole must allow them. If your fab wants 0.3 mm, switch to
  `QFN-60-1EP_7x7mm_P0.4mm_EP3.4x3.4mm` and add stitching vias by hand.
- **GPIO17–25 and 27–29 are unused** and carry no-connect flags. They are free if a later
  revision wants a header or a second target interface.
- **J1 USB-C edge position needs checking** against the receptacle actually bought — the
  mating face should overhang the board edge, possibly with an Edge.Cuts notch.
- **F1 needs a max-resistance check.** A 500 mA-hold PTC runs 0.4 Ω to 1.2 Ω depending on
  the part, which is 0.15 V to 0.5 V at full load, straight off the target rail. Pick from
  the low end.
- **TPS2114A is old but not end-of-life.** TI lists both the TPS2114A and TPS2115A as
  ACTIVE — in production and recommended for new designs. The problem is purely that JLCPCB
  does not carry it, so it is a distributor question, not a lifecycle one: buy it from
  Digi-Key or Mouser and the part is fine.

## Board sections

```
      USB-C          power         RP2354A       translators      DUT 2x5
   ┌──────────┬────────────────┬──────────────┬──────────────┬────────────┐
   │ J1 D1 F1 │ U2 3V3  U3 mux │ U1  Y1  L1   │ U4  U5  U6   │ J4 + 22R   │
   │          │ Q1 discharge   │ SW1 SW2  J2  │              │ + ESD      │
   └──────────┴────────────────┴──────────────┴──────────────┴────────────┘
```

Signal flows left to right. Everything right of the translators is referenced to `VTGT`;
everything left of them is 3.3 V. RP2354 GPIOs are not 5 V tolerant, so no target-facing
signal bypasses a translator.

## DUT header (J4) — SinoLink V2.2 pinout

```
 1 VTGT    2 TCK
 3 GND     4 TDI
 5 GND     6 TMS
 7 KEY     8 TDO / DATA
 9 nRST   10 GND
```

## Target supply (U3, TPS2114A)

Truth table, in the datasheet's own `D1, D0` column order:

| D1 | D0 | OUT | Mode |
|----|----|-----|------|
| 1 | 1 | Hi-Z | **off, the reset default** via R13/R14 10 k pull-ups |
| 1 | 0 | IN1 | 5.0 V |
| 0 | 1 | IN1 or IN2 | auto, picks whichever input is higher |
| 0 | 0 | IN2 | 3.3 V |

Note the pin/bit order: GPIO11 drives D0 and GPIO12 drives D1, so the firmware word is
`(D1 << 1) | D0`. Writing `D1=0, D0=1` gets you auto-switching, not a hard 5 V select.

`STAT` (GPIO14) is a supply-source indicator, not a fault flag. It is open-drain and pulls
low both when IN1 is selected **and** when OUT is Hi-Z, so it only disambiguates against the
mode firmware commanded. It is Hi-Z only while IN2 is selected.

Current limit is `I = 250 / R_ILIM`; R15 = 750 Ω gives 333 mA, which keeps probe plus target
inside a plain USB 500 mA budget. The part is a **TPS2114A**, not the TPS2115A: the '2115A's
specified ILIM adjustment range starts at 0.63 A, so 333 mA is off the bottom of it and the
limit accuracy is unspecified there. The two are pin-, package- and datasheet-identical, and
the '2114A's constant is half, putting 333 mA mid-range.

There is no series Schottky, so the target sees the full rail. Expect roughly 0.2 V below
nominal under load: the polyfuse F1 contributes 0.1 V to 0.4 V depending on the part fitted,
the mux another ~40 mV at 333 mA.

`VTGT` is capped at **5.25 V**, USB 5 V at +5 %. The translators' VCCB abs-max is 5.5 V, but
the binding limit is the ESD suppressors D4–D8: the H5VSD3B is specified to VRWM = 5 V with
VBR(min) = 6.2 V, so above ~5.25 V its leakage is unspecified and the margin to the
breakdown knee gets thin. **An external target above 5.5 V destroys U4–U6.**

## Target discharge (Q1)

`VTGT` is pulled down through R21 (150 Ω, 1206) by Q1. At 5 V that is 167 mW into a 250 mW
part, so it is rated for continuous assertion, but it is meant to be pulsed. R20 (100 k)
holds the gate low so the FET stays off while the RP2354 is in reset and its GPIOs are
inputs.

## GPIO map

| GPIO | Net | Goes to | State at reset |
|------|-----|---------|----------------|
| 0 / 1 | — | unused, no-connect. GPIO0 is `XIP_CS1n`, the chip select for a second QSPI memory, so keep it free | input, pulled down |
| 2–5 | TCK TDI TMS nRST | U4 A1–A4 | Hi-Z via OE |
| 6 | DATA | U5 A — PIO in/out base | input |
| 7 | DATA_DIR | U5 DIR — PIO side-set | low (B→A) |
| 8 / 9 | KEY / KEY_DIR | U6 A / DIR | low (B→A) |
| 10 | XLAT_OE | U4 pin 22 | high = disabled |
| 11 / 12 | VTGT_D0 / D1 | U3 pins 2, 3 | high, high = off |
| 13 | VTGT_DISCH | Q1 gate | low = off |
| 14 | VTGT_STAT | U3 pin 1 | input |
| 15 / 16 | LED_ACT / LED_ERR | D2 / D3 | off |
| 26 | VTGT_SENSE | ADC0, VTGT ÷ 2 | input |
| 17–25, 27–29 | — | unused, not brought out | input, pulled down |

GPIO 2–7 are contiguous on purpose: PIO wants TCK on side-set, TDI on out, TMS on set and
TDO on in, each with its own base register.

Every control line has its own pull resistor because RP2354 GPIOs are inputs out of reset.
Without them the target rail and the translators would come up in an undefined state.

## RP2354A notes

The RP2354A is an RP2350A with 2 MB of flash stacked in the same QFN-60, so there is no
external QSPI memory on this board. The QSPI pins are still bonded out and wired internally
to the flash die; they are left unconnected here. `QSPI_SS` is the exception — it is the
BOOTSEL strap, so SW1 still hangs off it through R7.

The core supply comes from the on-chip switching regulator, not RP2040's LDO. That needs
external parts and the layout is not optional:

- **L1 must be an Abracon AOTA-B201610S3R3-101-T** (3.3 µH, 0806). Raspberry Pi had this
  part made with a polarity dot specifically because the regulator's behaviour depends on
  which way the coil is wound — a "wrong way round" inductor couples into C21 and upsets the
  control loop. Pad 1 of the footprint is the dot; current must enter there, so pad 1 goes
  to `VREG_LX`.
- C19 (VREG_VIN), C21 (output) and C20 (VREG_AVDD) are 4.7 µF each.
- R5 (33 Ω) with C20 filters `VREG_AVDD`, which draws ~200 µA and is noise-sensitive.
- `VREG_PGND` carries the switching return current. It must reach GND without routing that
  current through the rest of the ground pour.

Follow the layout in the RP2350 hardware design guide for this block rather than improvising.

**Y1 must be an Abracon ABM8-272-T3** (12 MHz, CL 10 pF, ESR ≤ 50 Ω). The 1 kΩ series
resistor R6 and the 15 pF load caps are only validated for that crystal; substituting one
requires temperature testing.

## Debug

J2 is a **Tag-Connect TC2030-IDC-NL** footprint — no connector is fitted, the pogo-pin
cable lands directly on the pads.

```
 1 +3V3
 2 SWDIO
 3 nRESET  (RUN)
 4 SWCLK
 5 GND
 6 SWO     (not connected)
```

## Libraries

`sym-lib-table` and `fp-lib-table` are project-local and reference `${KICAD10_SYMBOL_DIR}` /
`${KICAD10_FOOTPRINT_DIR}`, so they survive nixpkgs updates. `nix develop` sets both.

`symbols/sinodude.kicad_sym` holds **TPS2114APW**, which is not in the stock libraries.
`footprints/sinodude.pretty` holds the polarised 0806 inductor land pattern.

## Checks

```sh
nix develop -c kicad-cli sch erc --severity-error --format report -o /tmp/erc.rpt sinodude-rp2354.kicad_sch
nix develop -c kicad-cli sch export netlist --format kicadsexpr -o /tmp/n.net sinodude-rp2354.kicad_sch
```
