#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::panic::PanicInfo;

use pic64gx::bringup_uart;
use riscv_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        spin_loop();
    }
}

#[entry]
fn main() -> ! {
    bringup_uart::init_8n1(bringup_uart::DEFAULT_BAUD);
    bringup_uart::write_str("\r\n[test2] MMUART2 init + TX smoke test\r\n");
    bringup_uart::write_str("[test2] Assumes 150 MHz UART input clock.\r\n");

    loop {
        spin_loop();
    }
}
