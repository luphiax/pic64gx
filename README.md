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
- `device.x`: minimal interrupt definitions for later `riscv-rt` integration

## Next step

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

Once the SVD is validated, run `./update.sh` to generate the PAC sources and
replace the placeholder `src/` tree.
