#[doc = "Register `DMR` reader"]
pub type R = crate::R<DmrSpec>;
#[doc = "Register `DMR` writer"]
pub type W = crate::W<DmrSpec>;
#[doc = "Field `DIVISOR_MSB` reader - Baud divisor most-significant byte"]
pub type DivisorMsbR = crate::FieldReader;
#[doc = "Field `DIVISOR_MSB` writer - Baud divisor most-significant byte"]
pub type DivisorMsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Baud divisor most-significant byte"]
    #[inline(always)]
    pub fn divisor_msb(&self) -> DivisorMsbR {
        DivisorMsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud divisor most-significant byte"]
    #[inline(always)]
    pub fn divisor_msb(&mut self) -> DivisorMsbW<'_, DmrSpec> {
        DivisorMsbW::new(self, 0)
    }
}
#[doc = "Divisor Latch Register (MSB) when LCR.DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`dmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmrSpec;
impl crate::RegisterSpec for DmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmr::R`](R) reader structure"]
impl crate::Readable for DmrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmr::W`](W) writer structure"]
impl crate::Writable for DmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMR to value 0"]
impl crate::Resettable for DmrSpec {}
