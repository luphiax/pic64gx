#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Field `WLS` reader - Word length select"]
pub type WlsR = crate::FieldReader;
#[doc = "Field `WLS` writer - Word length select"]
pub type WlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DLAB` reader - Divisor latch access bit"]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - Divisor latch access bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Word length select"]
    #[inline(always)]
    pub fn wls(&self) -> WlsR {
        WlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word length select"]
    #[inline(always)]
    pub fn wls(&mut self) -> WlsW<'_, LcrSpec> {
        WlsW::new(self, 0)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {}
