#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dlr: [u8; 0x04],
    dmr: Dmr,
    _reserved2: [u8; 0x04],
    lcr: Lcr,
    _reserved3: [u8; 0x04],
    lsr: Lsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Register (LSB) when LCR.DLAB is set"]
    #[inline(always)]
    pub const fn dlr(&self) -> &Dlr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Divisor Latch Register (MSB) when LCR.DLAB is set"]
    #[inline(always)]
    pub const fn dmr(&self) -> &Dmr {
        &self.dmr
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
}
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "DLR (rw) register accessor: Divisor Latch Register (LSB) when LCR.DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`] module"]
#[doc(alias = "DLR")]
pub type Dlr = crate::Reg<dlr::DlrSpec>;
#[doc = "Divisor Latch Register (LSB) when LCR.DLAB is set"]
pub mod dlr;
#[doc = "DMR (rw) register accessor: Divisor Latch Register (MSB) when LCR.DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`dmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmr`] module"]
#[doc(alias = "DMR")]
pub type Dmr = crate::Reg<dmr::DmrSpec>;
#[doc = "Divisor Latch Register (MSB) when LCR.DLAB is set"]
pub mod dmr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
