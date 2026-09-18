# sinodude

A programming tool for SinoWealth 8051 microcontrollers.

## Warning

**This tool is experimental. Use at your own risk.**

SinoWealth microcontrollers have limited public documentation. This tool has been developed through reverse engineering and may not work correctly with all chip variants and all configurations.

## Supported Programmers

| Programmer | Description | Notes |
|------------|-------------|-------|
| sinodude-serial | Open-source Arduino Nano (ATmega328P or ATmega328PB) based programmer. See [firmware/README.md](firmware/README.md) for details. | Recommended |
| sinolink | The genuine SinoWealth SinoLink USB dongle (`258a:5063` / `258a:5007`), driven over WinUSB-style vendor control transfers plus bulk endpoints. | See the limitations below |

## Using the SinoLink dongle

Pick the backend with `-c sinolink` and the target power with `--power`:

| `--power` | effect |
|---|---|
| `3v3` (default) | dongle supplies 3.3 V |
| `5v` | dongle supplies 5 V |
| `external` | load switch stays off; the board powers the target |

The selection is checked against the part's `compatible_voltages` and then **verified against the
dongle's ADC**, so asking for 5 V on a 3.3 V-only part is refused and a rail that does not measure
what was asked for is an error rather than a silent overvolt.

```sh
sinodude read     -c sinolink -p sh68f90a --power 3v3 --flash out.hex
sinodude write    -c sinolink -p sh68f90a --power 5v  --flash firmware.hex
sinodude erase    -c sinolink -p sh68f90a --power 3v3
sinodude security -c sinolink -p sh68f90a --power 3v3 --write_protect 3
```

`sinodude sinolink` exposes the dongle directly for diagnostics, below the part abstraction:

```sh
sinodude sinolink probe
sinodude sinolink connect --part sh68f90a --power 3v3
sinodude sinolink dump    --part sh68f90a --power 3v3 --addr 0x1000 --len 0x40 --region custom
sinodude sinolink erase   --part sh68f90a --power 3v3 --erase_mode mass
sinodude sinolink program --part sh68f90a --power 3v3 --bin image.bin --addr 0
sinodude sinolink recover --part sh68f90a --power 3v3   # erase + restore upper options in one session
```

### Limitations

**The per-chip config blob is only solved for SH68F90A.** The dongle latches ChipType, the link
class, geometry and the region addresses from a 1 KiB blob the host downloads at connect. Deriving
that blob from a part's parameters is unfinished, so every other part gets the captured SH68F90A
blob with a handful of fields patched, and the driver warns when that happens. Symptom of a wrong
blob is a read that returns one uniform byte for the whole address space.

**Erased code flash reads `0x00` on SH68F90A, not `0xFF`.** Do not blank-check against `0xFF`.

**Mass erase clears the code options.** It takes the whole part with it, including the boot region
and the custom/option region, so the options have to be written back afterwards. The `write`
subcommand does this for you: it snapshots the custom region before erasing and restores it as part
of the write.

**Program code options with `sinolink options`, never with a raw write.** The code options live in
the custom region, but writing them there with an ordinary program request only sets the bits the
chip lets you set; the rest silently refuse and the operation reports `0xcc`. `bRequest 0x11`
(`sinodude sinolink options`) stages them out of the config blob instead and sets every bit,
including the ones a raw write cannot.

**Result `0xcc` means the write did not verify**, not "transient, retry". The dongle programs, reads
back, and reports `0xcc` on a mismatch. Treat it as a refused write.

**The dongle wedges, and a wedged dongle lies to you.** After enough operations it starts failing
everything: `0x09` on connect, `0xcc` on every write, long chunked reads that disagree between runs.
None of that means the target is broken. Power-cycle the dongle's USB port and it comes back:

```sh
uhubctl -l <hub> -p <port> -a cycle -d 2      # find it with: uhubctl | grep 258a
```

Cycle it before trusting any negative result, and between long write sequences. Several findings in
this file had to be re-verified on a freshly cycled dongle before they could be believed.

**The custom page at `0x1000` holds several different fields; do not write it blind.** On an
SH68F90A the layout is CustomerID at `0x1000`, OperationNumber at `0x1004`, CustomerOption at
`0x1006` and the **security/protection record at `0x100a`**. Writing a 16-byte block at `0x1000`
therefore writes the protection record too. Bytes put there decode as per-group protection: writing
`0f 00 00 88` at `0x100a` marks group 0 protected, which shows up as a program failing at address 1,
and `00 00 00 88` marks groups 6 and 7.

Protection only takes effect with the flag byte `0x33` at record offset `0x10` plus the 6-byte mark,
so stray bits in the record decode as protected without actually blocking writes. Use the
`security` subcommand to check what is really in there.

**Writing the `0x1000` page erases code flash `0x0000-0x0fff`** as a side effect, verified with a
dongle power cycle between write and read. So write custom fields **before** the flash image, never
after. The `write` subcommand already orders it that way.

Mass erase clears the whole page, protection record included, so re-write what you need after any
erase-and-program cycle.

**Do not read immediately after a write.** The dongle answers from a stale internal buffer: a full
dump taken right after an option write returned the option bytes at address 0. The driver settles
for 20 ms after programming and after connecting, which was enough to make repeated full-flash
dumps byte-identical, but a verify pass is still worth doing in a separate session.

### Restore the upper code options in the same session as the erase

Code options are eight bytes: 0-3 at `CustomerOption` (`0x1006` on an SH68F90A) and 4-7 at a fixed
`0x1100`, both in the custom region. Byte 7 holds `OP_ISPSIZE` in its high nibble and
`OP_EEPROMSIZE` in its low nibble, inverse-coded so `8` means "reserve nothing". `SH68F90A.GPT`
gives `[EEPROM] 0`, `[ISPSIZE] 0` and `default_code_options[7] = 0x88`, so the whole 64 KiB is code
flash.

**Mass erase clears that page, and if byte 7 comes back `0x00` the part reserves 8 KiB of ISP plus
4 KiB of EEPROM and refuses to program above `0xd000`.** Erase therefore has to put the upper
options back, and this is the part that bites:

> The upper option page is only writable in the ICP session the erase leaves open. Reconnect first
> and the write fails with `0xcc` and nothing lands, every time. `mass_erase_with_mode` restores
> the options before it reconnects, mirroring `blank_security_and_set_code_option_defaults` in
> sinodude-serial, which is called from inside `mass_erase()` for the same reason.

Symptoms of getting this wrong: programming stops at `0xd000` with `0xcc`, the option page at
`0x1100` refuses writes at offset 3 while offsets 0, 1, 2 and 4 take, and the custom space behaves
oddly because the carve-out is live. All of it clears up once byte 7 reads `0x88`.

Two traps while debugging this. Writing **zeros** "succeeds" anywhere, because the echo of an erased
cell already matches, so a zero-fill probe proves nothing. And a **read taken soon after a write**
can return the dongle's stale buffer, so a program-then-verify inside one session can report success
for a region that was never written; power-cycle the dongle before trusting a negative result.
