#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Placeholder PAC tree.
//!
//! Run `./update.sh` after validating `pic64gx.svd` to generate the real
//! register access API with `svd2rust`.

pub mod bringup_uart;

#[cfg(feature = "rt")]
#[export_name = "_dispatch_exception"]
pub unsafe extern "C" fn dispatch_exception(trap_frame: &riscv_rt::TrapFrame, _code: usize) {
    unsafe extern "C" {
        fn ExceptionHandler(trap_frame: &riscv_rt::TrapFrame);
    }

    // The bring-up payload does not implement per-exception dispatch yet.
    // Trap into the runtime's default exception loop instead.
    unsafe { ExceptionHandler(trap_frame) }
}
