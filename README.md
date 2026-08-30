# RHAP-D for the Bitaxe Gamma

This firmware implements the Raw Hardware Access Protocol (RHAP) on the
ESP32-S3 of the Bitaxe Gamma. It passes ASIC UART, board I2C, GPIO, and
ADC through to a host over usbserial. It displaces the stock firmware,
esp-miner, and does no mining of its own.

RHAP-D is the device role of RHAP. The host role belongs to the program
at the other end of the USB connection, which drives the board through
this firmware. Mujina takes that role to run a Bitaxe as a hashboard.
Research, testing, and debugging use the same access.

This firmware continues [bitaxe-raw], the original implementation of
what has become RHAP. Unlike bitaxe-raw, which covers several boards,
this repository targets only one.

[bitaxe-raw]: https://github.com/bitaxeorg/bitaxe-raw

## Developing

The build needs rustup and [just]. Install rustup from
[rustup.rs], then install just:

```bash
cargo install just --locked
```

Run `just` with no arguments to list the recipes. Run `just checks`
before committing; it runs rustfmt and clippy.

[just]: https://just.systems/
[rustup.rs]: https://rustup.rs/

### Install the toolchain

```bash
just setup-tools
```

This installs [espup], the `esp` Rust toolchain, which has the
Xtensa backend, and [espflash]. `rust-toolchain.toml` selects the
`esp` toolchain inside this directory and leaves the default
toolchain alone elsewhere. Cargo links with a wrapper script in
`tools/` that finds the Xtensa GCC inside the toolchain, so no
shell setup is needed and espup's export file is discarded.

[espup]: https://github.com/esp-rs/espup
[espflash]: https://github.com/esp-rs/espflash

### Build the firmware

```bash
just build
```

### Flash the device

```bash
just flash
```

The board restarts into the new firmware when flashing completes.

Once this firmware runs, the ESP32-S3 must be put into the
bootloader before it can be flashed again. Hold the `BOOT` button
while attaching power.

## Running
When connected, this usbserial firmware will create two serial ports.
Usually the first serial port is "control serial" like I2C, GPIO, and
ADC. The second serial port is "data serial" and is pass through UART.

After startup, the ASIC is held in reset by GPIO RST_N, and the
firmware turns the TPS546 core voltage rail off, to minimize heat
and power until the host is connected and ready to use the ASIC.

### Data Serial
- Second serial port
- All data is passed through, both directions.
- usbserial baudrate is mirrored on the output.


### Control Serial
- First serial port
- baudrate does not matter

**Packet Format**

| 0      | 1      | 2  | 3   | 4    | 5   | 6... |
|--------|--------|----|-----|------|-----|------|
| LEN LO | LEN HI | ID | BUS | PAGE | CMD | DATA |

```
0. length low
1. length high
	- packet length is number of bytes of the whole packet. 
2. command id
	- Whatever byte you want. will be returned in the response 
3. command bus
	- always 0x00 
4. command page
	- I2C:  0x05
	- GPIO: 0x06
	- ADC:  0x07
5. command 
	- varies by command page. See below
6. data
	- data to write. variable length. See below
```

**I2C**

Commands:

- write: 0x20
- read: 0x30
- readwrite: 0x40

Data:

- [I2C address, (bytes to write), (number of bytes to read)]

Example:

- write 0xDE to addr 0x4F: `08 00 01 00 05 20 4F DE`
- read one byte from addr 0x4C: `08 00 01 00 05 30 4C 01`
- readwrite two bytes from addr 0x32, reg 0xFE: `09 00 01 00 05 40 32 FE 02`

**GPIO**

Commands:

- RST_N: 0x00

Data:

- [pin level]

Example

- Set pin 1 Low: `07 00 00 00 06 01 00`

**ADC**

Commands:

- read VDD: 0x50
- read VIN: 0x51

Example:

- read VDD Pin: `06 00 00 00 07 50`

