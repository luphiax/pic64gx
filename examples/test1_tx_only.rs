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
    bringup_uart::write_str("\r\n[test1] MMUART2 TX-only smoke test\r\n");
    bringup_uart::write_str("[test1] Assumes previous firmware already configured UART2.\r\n");

    loop {
        spin_loop();
    }
}
