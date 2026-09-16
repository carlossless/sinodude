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
| Schematic | complete — 93 components, ERC clean |
| Sourcing | 88 of 93 parts carry an `LCSC` field; the 5 without are mounting holes and Tag-Connect pads |
| PCB | complete — 64 × 36 mm, two layers, fully routed, DRC clean |

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
R5   33R/1%/R0402                        F1       750mA/Rmax290mR/F1206
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
| J1 | GCT USB4105-GF-A-120 | `C5184243` | swapped from the HCTL part, see below |
| J4 | ZHOURI DC3-2.54-10PAL | `C5156674` | right-angle shrouded box header |
| SW1, SW2 | C&K KMR221GLFS | `C72443` | the part the KiCad footprint is named after |

**Watch the DC3 suffix when reordering J4.** The family splits on one letter: `-10PAS` is
`Plugin`, i.e. vertical, and `-10PAL` is `Push-Pull`, i.e. **Right Angle**. Only the PAL
fits `IDC-Header_2x05_P2.54mm_Horizontal`. Ordering the PAS gives you a connector that
physically fits the same holes but points the cable straight up off the board.

**U2 is now an AMS1117-3.3** (`C6186`, Basic, ~1.4 M in stock), not the NCP1117 the RP2350
guide uses — NCP1117 is not stocked at JLCPCB. Same SOT-223 land pattern, same pinout
(1 GND, 2 VO, 3 VI).

One part-choice consequence worth knowing:

- **C27 (10 µF on VTGT) slows supply switching.** The TPS211xA will not connect a supply
  until OUT has fallen within 100 mV of it, so on a 5 V → 3.3 V change VTGT must decay to
  ~3.4 V first. TI notes the fast input-to-input slew "reduces the output voltage hold-up
  capacitance requirement". Q1 is therefore not optional for voltage changes, not just for
  ICP power cycling — or drop C27 to 1–4.7 µF.

### 3D models

Every footprint representing a physical part resolves a 3D model — 88 of the 93. The five
without are the Tag-Connect pad pattern and the four mounting holes, none of which is a
component.

Check it with `kicad-cli pcb export step`, not by eye. A footprint can carry a model in the
library and still have none on the board, because the board keeps its own copy of the
footprint; U1 sat like that for a while, model file present, library entry present, board
instance empty.

`flake.nix` exports `KICAD10_3DMODEL_DIR` — without it none of the stock models resolve at
all, since every footprint references that variable.

Two models are generated locally into `3dmodels/`, because the KiCad library ships neither:

| Model | Why |
|---|---|
| `QFN-60-1EP_7x7mm_P0.4mm_EP3.4x3.4mm.step` | the KiCad 3D library has QFN-52 and QFN-56 in 7×7 but **no QFN-60**, so the RP2354A had no body at all |
| `L_Abracon_AOTA-B201610S_0806_2016Metric.step` | matches the custom inductor footprint |

Both were built in FreeCAD to datasheet nominals — the QFN to RP2350 datasheet Figure 143
(D/E 7 BSC, A 0.85, A1 0.02, A3 0.203 REF, D2/E2 3.40, b 0.18, e 0.400 BSC, L 0.40) and the
inductor to Abracon's 2.00 × 1.60 × 1.00 max. Verified bounding boxes are 7.000 × 7.000 ×
0.850 and 2.000 × 1.600 × 1.000, origin centred, seating plane at z=0.

Each is exported as separate solids so it can carry per-solid STEP colour, using the same
three RGB values the stock KiCad models use (body `0.148/0.145/0.145`, leads and terminals
`0.824/0.820/0.781`, pin-1 and polarity marking `0.691/0.664/0.598`). Without that a
generated model renders flat grey next to every other part on the board.

They are visual and mechanical-fit models, not vendor CAD: lead geometry is nominal and
there is no internal detail. Good enough for enclosure fit and collision checks; if you need
certified geometry, replace them with vendor STEP.

Because a 3D model is attached to a footprint, `QFN-60-…_ThermalVias` is **copied** into
`footprints/sinodude.pretty` with its model path retargeted. Its nine thermal vias are also
widened from 0.5/0.2 mm to 0.6/0.3 mm, which is JLCPCB's minimum drill.

`footprints/sinodude.pretty` holds two more project copies, each because the stock footprint
does not fit this board:

| Footprint | Change |
|---|---|
| `USB_C_Receptacle_GCT_USB4105-xx-A_16P_TopMnt_Horizontal` | the four corner ground pads are 1.00 mm tall instead of 1.15 mm, so they clear the receptacle's own shell holes at a 0.25 mm hole-to-copper rule |
| `IDC-Header_2x05_P2.54mm_Horizontal` | the pin 1 arrow is shortened so it does not sit on R26's pad, and the body outline is clipped at the board edge it overhangs |

Copies do not pick up upstream library fixes. The trade is deliberate: with them, every
footprint on the board matches its library and DRC's library check is silent, so a real
drift will show up.

**J1 moved to the GCT USB4105** (`C5184243`, USB4105-GF-A-**120**, 16P, 4143 in stock). The
HCTL footprint referenced a model the library does not ship. GCT has an identical pad set
(A1 A4–A9 A12, B1 B4–B9 B12, SH), so this was a footprint swap with no schematic change, and
it does have a model. Note the `-120` suffix: the plain `USB4105-GF-A` and `-060` are 12-pin
parts and will not fit.

### Known items

- **The ERC pin matrix allows a bidirectional pin on a power-output net.** U4's unused pins
  (7–10 = A5–A8, 14–17 = B5–B8) are tied to GND, and GND carries a PWR_FLAG, so the stock
  matrix flags "bidirectional pin connected to power output" eight times. The wiring is
  right and KiCad only sees the pin *type*; it cannot know DIR is strapped. Rather than
  carry eight standing warnings, the Bidirectional × Power-output cell is set to "no
  warning" in the project's ERC matrix. ERC is clean; if you add a real bidirectional pin
  to a driven rail, that pair will no longer be caught for you.

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
- **U1 uses the ThermalVias QFN footprint** (vias under the exposed pad). The project copy
  drills them at 0.3 mm rather than the stock 0.2 mm, so they are inside JLCPCB's standard
  capability; if you retarget to a fab with a finer drill there is nothing to change.
- **GPIO17–25 and 27–29 are unused** and carry no-connect flags. They are free if a later
  revision wants a header or a second target interface.
- **Both board edges are cut back from where the footprints put them**, so the connectors
  reach through a case wall. The GCT footprint's `PCB Edge` line on `Dwgs.User` sits flush
  with the receptacle's mating face; the Edge.Cuts line is 1.4 mm inboard of it. J4 gets the
  same treatment at the other end. If you ever move J1 or J4, the edges move with them.
- **F1 is a 750 mA part, not 500 mA.** Worst case through it is U3's 333 mA target limit
  plus the board's own draw, call it 400–430 mA. Against a 500 mA hold that is under 20%
  margin before derating, and PPTC hold current falls with ambient while U2 sits beside it
  burning ~0.7 W at that current, so it would nuisance-trip. The fitted part is now
  Littelfuse `1206L075/16WR` (`C371166`): 750 mA hold, 1.5 A trip, 90 mΩ typical and 290 mΩ
  after a trip, in the same 1206 land. Note the stock — about 6 k against 300 k for the
  500 mA Jinrui part it replaced. F1 guards the host port against a board fault; the
  target's own overcurrent limit is U3, so raising F1 does not weaken that.

  A **TI TPD3S014** (`C87384`, SOT-23-6) was weighed and turned down. It would replace both
  F1 and D1 with one part, limit electronically at a guaranteed 0.67 A rather than tripping
  thermally, and drop 42–56 mV instead of up to 125 mV — but it costs about a dollar more
  per board against two commodity parts, it is an Extended part at JLCPCB, and its 0.5 A
  continuous rating leaves only 14% over our worst case. Glasgow revC3 does use one, with no
  polyfuse at all. Do not reach for the TPD3S**044** if you revisit this: its limit is
  guaranteed only above 1.60 A, so a fault would pull more than a 500 mA host allows before
  it acted.
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

## Indicators (D2, D3)

Both run from `+3V3` through 470 Ω into the GPIO, so the pin sinks and both are active-low.

**Do not size these by current.** The two LEDs are binned at different test currents, and
reading the LCSC catalogue line instead of the datasheet gets this backwards:

| | part | Iv (bin range) | Vf | at | efficacy |
|---|---|---|---|---|---|
| D2 green, 517 nm | `C7496818` | 160–230 mcd | 2.6–2.9 V | IF = 5 mA | 32–46 mcd/mA |
| D3 red, 625 nm | `C2286` | 145–300 mcd | 1.8–2.4 V | IF = 20 mA | 7.3–15 mcd/mA |

At their respective test points the green is three to four times more efficient per
milliamp, and both are 120° parts, so the mcd figures compare directly for perceived
brightness. That is why the green gets the same resistor as the red rather than a smaller
one: on a 3.3 V rail its higher Vf holds it near 1 mA while the red sits near 3 mA, which
is roughly the four-to-one current ratio the efficacy difference calls for. Fitting the
green with, say, 150 Ω to "match currents" would make it several times brighter, not equal.

**The match cannot be predicted more precisely than that, and the datasheets are the
reason.** Anyone tempted to compute an exact figure should know:

- The green's characteristic curves both **start at 5 mA**. It runs here near 1 mA, five
  times below any published point, and InGaN greens are least predictable at the bottom of
  their range.
- The green's table and its own curves **disagree at the one current they share**. At 5 mA
  the table gives Iv 160–230 mcd and Vf 2.6–2.9 V; the curves read about 310 mcd and
  2.48 V, the latter below the table's own minimum.
- The red is strongly **super-linear at low current**: its relative-intensity curve reads
  about 0.45 at 5 mA against 1.0 at 20 mA, where linear scaling would give 0.25. Any
  straight-line extrapolation from the 20 mA test point understates it by nearly 2×.
- **Neither part is ordered to a bin.** The red alone ships across four Iv bins spanning
  145–300 mcd and six Vf bins spanning 1.8–2.4 V, so a 2:1 brightness spread exists before
  the two parts are compared at all.

Treat both resistors as bring-up items. They are 0603, the ratio above is the right
starting point, and the first assembled board settles it in a way no datasheet here can.

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

## PCB

64 x 36 mm, **two layers**, 2 mm corner radius, four M3 holes. Parts on both sides.
Fully routed in 919 segments and 264 vias; `kicad-cli pcb drc` and `kicad-cli sch erc` both
report nothing at any severity, and every track runs at 0, 45 or 90 degrees.

**Both connectors overhang the board, for a case.** The west edge is cut back so J1's shell
stands 1.4 mm proud, and the east edge so J4's shroud stands 2.1 mm proud. J1's 1.4 mm is
the geometric maximum, not a choice: its own shield tabs are plated slots 1.7 mm behind the
mating face, and the board has to hold them with 0.3 mm of copper clearance. J4's overhang
is free to grow — its pins stop 9.9 mm short of its front face, so the only limit is how
much unsupported shroud you want.

The four M3 holes sit 3.2 mm in from both edges they meet, so each is on the corner
diagonal. That 3.2 mm is set by U2, whose courtyard is what H3 would run into first.

J1 (USB-C), U1 and J4 (DUT header) share the y = 120 centreline, and the signal flow runs
left to right along it: USB-C, ESD, MCU, level shifters, DUT header. The board is laid out
in blocks: USB and the 3V3 regulator west, SWD connector and clock north, target supply and
level shifters east, the target discharge switch tucked under U6 beside the VTGT network it
switches, the two buttons together along the south edge below U4.

No part sits out on its own against an edge. The four that did have been brought in next to
what they serve: C26 under U2's 3V3 output, R8 onto the `+3V3` run it pulls `RUN` up to,
R15 beside U3's ILIM pin, and C29 next to the VTGT sense divider.

**U1 is rotated 180 degrees**, which puts its east column against U4 and brings USB DM/DP
out of the south row toward J1.

**U1's GPIO assignment is chosen for the layout.** The east column presents the JTAG group
as TCK, TDI, TMS, NRST going south, matching U4's A-side and the DUT header pin order, and
DATA sits north of KEY to match U5 and U6. Assigned the other way round these two groups
have to cross inside the 0.4 mm pad ring, where there is no room; the nets are plain GPIOs,
so the crossing is designed out rather than routed around.

**Decoupling and pull-ups live on the bottom**, directly under the IC they serve. On two
layers this is what buys the design its routing room: the top side stays clear for signals,
and each decoupling cap reaches its power pin through a via rather than around the package.
Series resistors in a signal path (R3/R4 on USB, R25-R30 and R32 to the DUT) stay on top
where the trace already runs.

GND pours on both layers with solid pad connections (thermal spokes starve on two layers)
and roughly 170 vias into them, on a 2.5 mm lattice wherever both pours are free, including
the nine under U1's exposed pad. Isolated pour islands are dropped rather than left
floating.

**The ground net is one piece of copper, and that took work.** On two layers a dense signal
field cuts the pour into pockets, and a pocket that holds a ground pad but reaches nothing
else leaves that pad floating while DRC still shows the net as routed: the ratsnest line
runs zone-to-zone, which is easy to wave away. Every pocket here is stitched back, and three
needed a local change rather than a via — C32 and U5's ground pin came free once `DATA_DIR`
was rerouted, and C12's ground pad only opened up after the `+3V3` climb between C6 and C12
moved out of the way and the two caps' ground pads were tied to each other directly.

No via lands on a Tag-Connect pad, and J2 now carries the stock footprint's keepout zone
(no vias, no pour) over its six pads, so the rule is enforced by the footprint rather than
by remembering it.

**The Tag-Connect (J2) is routed by hand.** Its five leg holes are exported as keepout
circles that box in each pad, leaving exactly one single-track channel per pad, and which
channel a pad may use is forced by geometry. No autorouter finds these. The same applies to
the VBUS bridge between J1's two power pad pairs, which crosses on the back so the CC and
D+/D- pads keep the front-side corridor beside the connector.

A pogo pin has to land on bare, flat copper, and a via in the pad wicks paste and leaves a
dimple even when it carries the same net, which no clearance rule catches.

`USB_DM`/`USB_DP` run as a coupled pair from J1 through R3/R4 to U1, 0.15 mm wide on a
0.25 mm gap, on the front layer with no via between connector and MCU. Full-speed USB does
not need this; the pair is routed anyway so the segment between the ESD diode and the
series resistors has a defined return path.

### Design rules

Derived from Glasgow revC3, scaled for two layers. These are JLCPCB's standard tier, so the
board does not attract fine-pitch pricing.

| Class | Track | Clearance | Via |
|---|---|---|---|
| Default | 0.15 mm | 0.15 mm | 0.6 / 0.3 mm |
| Power | 0.30 mm | 0.15 mm | 0.6 / 0.3 mm |
| USB | 0.15 mm | 0.15 mm | 0.6 / 0.3 mm, pair gap 0.25 mm |

Board minimums are set to JLCPCB's two-layer capability: 0.127 mm track and clearance,
0.3 mm drill, 0.45 mm via, 0.13 mm annular ring, 0.5 mm hole-to-hole, 0.3 mm copper-to-edge,
0.8 mm silkscreen text at 0.15 mm line width.

RP2350's USB is full-speed only, so the USB class holds a pair geometry but no target
impedance, and two layers cost nothing here.

One 4.2 mm stretch of the `+3V3` run east of U4 is 0.20 mm rather than the class's 0.30 mm.
It is the rail into U5/U6's A-side supply, a few milliamps, and the 0.10 mm it gives up is
the slot U5's ground pin escapes through.

### Silkscreen

Every reference designator is clear of pads and of other silkscreen, and reads upright from
its own side of the board — the back is upright when you flip the board over, so it reads
correctly in a mirrored plot. Designators sit next to their part rather than in a fixed
position, so a few are on the far side of the part from where the footprint puts them by
default.

Values are hidden except on the two buttons, where SW1 and SW2 carry `BOOTSEL` and `RESET`
below them — the Value field, not a free-standing legend, so the silk cannot drift from the
schematic. The mounting holes are labelled H1–H4 on the inboard side of each hole.

### Passive sizes

Resistors and capacitors are **0603** throughout, with three deliberate exceptions where the
package is doing electrical work:

| Part | Size | Why not 0603 |
|---|---|---|
| R21 | 1206 | dissipates ~167 mW discharging VTGT from 5 V; 0603 is rated 100 mW |
| C5 | 1206 | 22 µF bulk; DC-bias loss in a smaller case would eat the capacitance |
| C4, C27 | 0805 | 10 µF bulk, same reason |

J2 is the **latching** Tag-Connect (TC2030-IDC-FP): the cable's legs clip through four
2.37 mm holes, so it stays put unattended. The no-legs variant has to be held by hand.

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
