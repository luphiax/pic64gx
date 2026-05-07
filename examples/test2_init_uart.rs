#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::panic::PanicInfo;

use pic64gx::Uart2;
use riscv_rt::entry;

const UART_INPUT_CLOCK_HZ: u32 = 150_000_000;
const DEFAULT_BAUD: u32 = 115_200;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        spin_loop();
    }
}

const fn baud_divisor(clock_hz: u32, baud: u32) -> u16 {
    ((clock_hz + (baud * 8)) / (baud * 16)) as u16
}

fn tx_ready(uart: &Uart2) -> bool {
    uart.lsr().read().thre().bit_is_set()
}

fn wait_for_tx_ready(uart: &Uart2) {
    while !tx_ready(uart) {
        spin_loop();
    }
}

fn write_byte(uart: &Uart2, byte: u8) {
    wait_for_tx_ready(uart);
    uart.thr().write(|w| unsafe { w.data().bits(byte) });
}

fn write_str(uart: &Uart2, s: &str) {
    for &byte in s.as_bytes() {
        write_byte(uart, byte);
    }
}

fn init_8n1(uart: &Uart2, baud: u32) {
    let divisor = baud_divisor(UART_INPUT_CLOCK_HZ, baud);
    let dlr = (divisor & 0x00ff) as u8;
    let dmr = (divisor >> 8) as u8;

    uart.lcr().write(|w| w.dlab().set_bit());
    uart.dlr().write(|w| unsafe { w.divisor_lsb().bits(dlr) });
    uart.dmr().write(|w| unsafe { w.divisor_msb().bits(dmr) });
    uart.lcr().write(|w| unsafe { w.wls().bits(0b11) });
}

#[entry]
fn main() -> ! {
    let uart = unsafe { Uart2::steal() };

    init_8n1(&uart, DEFAULT_BAUD);
    write_str(
        &uart,
        "Hello World from a baremetal program on a PIC64GX1000 Curiosity Kit ES\r\n",
    );

    loop {
        spin_loop();
    }
}
