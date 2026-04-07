use core::hint::spin_loop;
use core::ptr::{read_volatile, write_volatile};

pub const MMUART2_BASE: usize = 0x2010_2000;
pub const UART_INPUT_CLOCK_HZ: u32 = 150_000_000;
pub const DEFAULT_BAUD: u32 = 115_200;

const THR_ADDR: *mut u32 = (MMUART2_BASE + 0x00) as *mut u32;
const DLR_ADDR: *mut u32 = (MMUART2_BASE + 0x00) as *mut u32;
const DMR_ADDR: *mut u32 = (MMUART2_BASE + 0x04) as *mut u32;
const LCR_ADDR: *mut u32 = (MMUART2_BASE + 0x0C) as *mut u32;
const LSR_ADDR: *const u32 = (MMUART2_BASE + 0x14) as *const u32;

const LCR_WLS_8BIT: u32 = 0b11;
const LCR_DLAB: u32 = 1 << 7;
const LSR_THRE: u32 = 1 << 5;

#[inline(always)]
fn read32(addr: *const u32) -> u32 {
    unsafe { read_volatile(addr) }
}

#[inline(always)]
fn write32(addr: *mut u32, value: u32) {
    unsafe { write_volatile(addr, value) }
}

#[inline(always)]
pub fn tx_ready() -> bool {
    (read32(LSR_ADDR) & LSR_THRE) != 0
}

#[inline(always)]
pub fn wait_for_tx_ready() {
    while !tx_ready() {
        spin_loop();
    }
}

#[inline(always)]
pub fn write_byte(byte: u8) {
    wait_for_tx_ready();
    write32(THR_ADDR, u32::from(byte));
}

pub fn write_bytes(bytes: &[u8]) {
    for &byte in bytes {
        write_byte(byte);
    }
}

pub fn write_str(s: &str) {
    write_bytes(s.as_bytes());
}

pub const fn baud_divisor(clock_hz: u32, baud: u32) -> u16 {
    ((clock_hz + (baud * 8)) / (baud * 16)) as u16
}

pub fn init_8n1(baud: u32) {
    let divisor = baud_divisor(UART_INPUT_CLOCK_HZ, baud);
    let dmr = u32::from((divisor >> 8) as u8);
    let dlr = u32::from((divisor & 0x00ff) as u8);

    write32(LCR_ADDR, LCR_DLAB);
    write32(DLR_ADDR, dlr);
    write32(DMR_ADDR, dmr);
    write32(LCR_ADDR, LCR_WLS_8BIT);
}
