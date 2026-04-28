#[doc = r" Core interrupts. These interrupts are handled by the core itself."]
# [riscv :: pac_enum (unsafe CoreInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreInterrupt {
    #[doc = "3 - Machine Software Interrupt"]
    MachineSoft = 3,
    #[doc = "7 - Machine Timer Interrupt"]
    MachineTimer = 7,
    #[doc = "11 - Machine External Interrupt"]
    MachineExternal = 11,
}
pub use riscv::interrupt::Exception;
#[doc = r" HARTs in the device"]
# [riscv :: pac_enum (unsafe HartIdNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hart {
    #[doc = "4 - Hart 4 (u54_4)"]
    H4 = 4,
}
pub use riscv::{
    interrupt::{disable, enable, free, nested},
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
