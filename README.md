# `pic64gx`

Bootstrap repository for a minimal PIC64GX PAC and baremetal Rust bring-up.

The initial goal is intentionally narrow:

- run on `u54_4`
- load at `0x91C00000`
- print a hello world on `UART2` (`0x20102000`)

This first step mirrors the structure of `riscv-rust/e310x`, but the SVD is
deliberately trimmed down to only the TX-only UART path needed for the first
serial bring-up.

## Current assumptions

- HSS still loads the payload to `0x91C00000`
- Linux still leaves `cpu4`, `mmuart2`, and the `0x91C00000..0x91CFFFFF`
  carveout untouched
- `UART2` is the serial port reserved for the standalone payload
- the register naming follows the PIC64GX `MMUART2_LO` register map directly

If the first baremetal hello world does not print, the next likely missing
piece is not more UART registers, but platform setup outside the current SVD
scope, such as clock/reset ownership.

## Layout

- `pic64gx.svd`: minimal device description for `UART2`
- `settings.yaml`: `svd2rust` settings for a single RV64 hart bring-up
- `update.sh`: regeneration script modeled after `e310x/update.sh`
- `memory.x`: linker memory map matching the current HSS payload carveout
- `link.x`: vendored runtime linker script for the high-address payload layout
- `device.x`: generated interrupt definitions used by the PAC runtime path
- `src/`: generated PAC sources for the current minimal `UART2` SVD

`update.sh` regenerates the PAC from `pic64gx.svd`, formats the generated
sources, and combines the generated interrupt definitions with `memory.x`.

## Current SVD Scope

The current SVD intentionally contains only:

- `THR` to transmit a byte
- `LSR` to poll for transmitter readiness
- `LCR`, `DLR`, and `DMR` to perform minimal baud and 8-bit word-length setup
  without relying on previous firmware state

It intentionally omits:

- receive path registers
- interrupt metadata and PLIC integration
- modem control/status
- FIFO control
- scratch or auxiliary UART registers
- every other PIC64GX peripheral

If you later decide to rely on preconfigured UART state from earlier firmware,
the SVD can be cut down even further to just `THR` and `LSR`.

## UART smoke tests

This branch carries one baremetal bring-up example:

- `examples/test2_init_uart.rs`
  - uses the generated PAC API for `UART2`
  - sets `LCR.DLAB`, programs `DLR/DMR`, restores `LCR` to 8-bit mode
  - assumes a `150 MHz` UART input clock

Current status:

- `cargo check --features rt --example test2_init_uart` passes on
  `riscv64gc-unknown-none-elf`
- full `cargo build --features rt --example test2_init_uart` now links at
  `0x91C00000` with the checked-in stable toolchain

The original linker failure was not caused by the UART register model. It came
from an implicit `.eh_frame` placement that made an `R_RISCV_32_PCREL`
relocation go out of range at the high payload address. The vendored `link.x`
keeps `.eh_frame_hdr` and `.eh_frame` in `REGION_RODATA`, near the rest of the
firmware image linked at `0x91C00000`.
