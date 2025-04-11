#[doc = "Register `SCU37C` reader"]
pub type R = crate::R<Scu37cSpec>;
#[doc = "Register `SCU37C` writer"]
pub type W = crate::W<Scu37cSpec>;
#[doc = "Field `PphaseCountingValueForRGMIITXCK` reader - P-phase counting value for RGMIITXCK"]
pub type PphaseCountingValueForRgmiitxckR = crate::FieldReader<u16>;
#[doc = "Field `NphaseCountingValueForRGMIITXCK` reader - N-phase counting value for RGMIITXCK"]
pub type NphaseCountingValueForRgmiitxckR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - P-phase counting value for RGMIITXCK"]
    #[inline(always)]
    pub fn pphase_counting_value_for_rgmiitxck(&self) -> PphaseCountingValueForRgmiitxckR {
        PphaseCountingValueForRgmiitxckR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - N-phase counting value for RGMIITXCK"]
    #[inline(always)]
    pub fn nphase_counting_value_for_rgmiitxck(&self) -> NphaseCountingValueForRgmiitxckR {
        NphaseCountingValueForRgmiitxckR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "Clock Duty Measurement Result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu37c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu37c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu37cSpec;
impl crate::RegisterSpec for Scu37cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu37c::R`](R) reader structure"]
impl crate::Readable for Scu37cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu37c::W`](W) writer structure"]
impl crate::Writable for Scu37cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU37C to value 0"]
impl crate::Resettable for Scu37cSpec {}
