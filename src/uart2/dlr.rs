#[doc = "Register `DLR` reader"]
pub type R = crate::R<DlrSpec>;
#[doc = "Register `DLR` writer"]
pub type W = crate::W<DlrSpec>;
#[doc = "Field `DIVISOR_LSB` reader - Baud divisor least-significant byte"]
pub type DivisorLsbR = crate::FieldReader;
#[doc = "Field `DIVISOR_LSB` writer - Baud divisor least-significant byte"]
pub type DivisorLsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Baud divisor least-significant byte"]
    #[inline(always)]
    pub fn divisor_lsb(&self) -> DivisorLsbR {
        DivisorLsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud divisor least-significant byte"]
    #[inline(always)]
    pub fn divisor_lsb(&mut self) -> DivisorLsbW<'_, DlrSpec> {
        DivisorLsbW::new(self, 0)
    }
}
#[doc = "Divisor Latch Register (LSB) when LCR.DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlrSpec;
impl crate::RegisterSpec for DlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlr::R`](R) reader structure"]
impl crate::Readable for DlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dlr::W`](W) writer structure"]
impl crate::Writable for DlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLR to value 0"]
impl crate::Resettable for DlrSpec {}
