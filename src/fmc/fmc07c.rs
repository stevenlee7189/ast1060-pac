#[doc = "Register `FMC07C` reader"]
pub type R = crate::R<Fmc07cSpec>;
#[doc = "Register `FMC07C` writer"]
pub type W = crate::W<Fmc07cSpec>;
#[doc = "Field `FIFOLengthStatus` reader - FIFO Length Status"]
pub type FifolengthStatusR = crate::FieldReader;
#[doc = "Field `FIFOLengthStatus` writer - FIFO Length Status"]
pub type FifolengthStatusW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - FIFO Length Status"]
    #[inline(always)]
    pub fn fifolength_status(&self) -> FifolengthStatusR {
        FifolengthStatusR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - FIFO Length Status"]
    #[inline(always)]
    pub fn fifolength_status(&mut self) -> FifolengthStatusW<Fmc07cSpec> {
        FifolengthStatusW::new(self, 0)
    }
}
#[doc = "DMA Buffer Mode Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc07cSpec;
impl crate::RegisterSpec for Fmc07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc07c::R`](R) reader structure"]
impl crate::Readable for Fmc07cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc07c::W`](W) writer structure"]
impl crate::Writable for Fmc07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC07C to value 0"]
impl crate::Resettable for Fmc07cSpec {}
